mod server_lib;

use std::sync::mpsc::{channel, Sender, Receiver};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;
use std::io::prelude::*;
use serde_json::{Value};


use server_lib::*;

pub fn create_game(tcp_stream: TcpStream) {
    println!("Game start with players {:?}", tcp_stream);
    
    let (to_client_1, from_server_1) = channel::<String>();
    let (to_client_2, from_server_2) = channel::<String>();
    let (to_server, from_client) = channel::<String>();

    let to_server_1 = to_server.clone();
    let to_server_2 = to_server.clone();

    thread::spawn(move || {
        player_thread(from_server_1, to_server_1.clone(), tcp_stream);        
    });
    thread::spawn(move || {
        ai_thread(from_server_2, to_server_2.clone());        
    });

    let game = GameState::new(to_client_1, to_client_2, from_client);
    game.run_game();

}

pub fn player_thread(from_server: Receiver<String>, to_server: Sender<String>,mut tcp_stream: TcpStream) {
    println!("Player therad starting");
    let _ = tcp_stream.set_nonblocking(true);
    let mut live_buffer = vec![];
    loop {
        let result = from_server.try_recv();
        match result {
            Ok(val) => {
                let _ = tcp_stream.write(val.as_bytes());
                let _ = tcp_stream.write("@@".as_bytes());
            },
            Err(_) => {

            }
        }
        let mut read_buffer = [0; 128];

        let number = tcp_stream.read(&mut read_buffer);
        match number {
            Ok(number) => {
                if number > 0 {
                    let mut new_vec = read_buffer[0..number].to_vec();
                    live_buffer.append(&mut new_vec);
                    
                    let mut has_found_first_part = false;
                    for i in 0..live_buffer.len() {
                        if live_buffer[i] as char == '@' {
                            if has_found_first_part == false {
                                has_found_first_part = true;
                            }
                            else {
                                let use_values : Vec<u8> = live_buffer.drain(0..i+1).collect();
                                let result = str::from_utf8(&use_values[0..i-1]).unwrap();
                                let _ = to_server.send(String::from(result));
                                //self.handle_rune(String::from(result));
                                break;
                            }
                        }
                    }
                }
            },
            Err(_) => {

            }
        }
            thread::yield_now();
    }
}

pub fn ai_thread(from_server: Receiver<String>, to_server: Sender<String>) {
    loop {
       let result = from_server.try_recv();

        match result {
            Ok(val) => {
                let v : Value = serde_json::from_str(&val).unwrap();
                let as_obj = v.as_object().unwrap();
                let rune_type = as_obj.get("rune_type").unwrap().as_str().unwrap();
                if rune_type == "report_options" {
                    let ro : ReportOptionsRune = serde_json::from_str(&val).unwrap();
                    'outer: for x in 0..19 {
                        for y in 0..19 {
                            if ro.board[x][y] {
                                let po : PickOption = PickOption::new(x, y);
                                let _ = to_server.send(po.to_string());
                                break 'outer;
                            }
                        }
                    }
                }
            },
            Err(_) => {
                
            }
        }
        thread::yield_now();
    }
}

pub fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    for stream in listener.incoming() {
        create_game(stream.unwrap());
        thread::yield_now();
    }
}