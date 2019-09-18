
use std::net::TcpStream;
use std::net::TcpListener;
mod server_lib;
use server_lib::*;
use std::thread;

pub fn create_game(tcp_stream: TcpStream) {
    println!("Game start with players {:?}", tcp_stream);
    thread::spawn(move || {
        let game = GameState::new(vec![tcp_stream]);
        game.run_game();
    });
}

pub fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    for stream in listener.incoming() {
        create_game(stream.unwrap());
    }
}