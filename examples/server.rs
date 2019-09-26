mod server_lib;

use GoBooM::*;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;

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
        let mut buffer = [0; 128];

        let read_bytes = tcp_stream.read(&mut buffer);
        match read_bytes {
            Ok(val) => {

            },
            Err(_) => {

            }
        }
//        let result = tcp_stream.read();
    }
}

pub fn ai_thread(from_server: Receiver<String>, to_server: Sender<String>) {
    loop {
       let result = from_server.try_recv();
        match result {
            Ok(val) => {

            },
            Err(_) => {
                
            }
        }
    }
}

pub fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    for stream in listener.incoming() {
        create_game(stream.unwrap());
    }
}