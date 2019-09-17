use std::thread;
use std::sync::mpsc::channel;
use std::net::TcpStream;
use std::net::TcpListener;
mod server_lib;
use server_lib::*;

pub fn create_game(tcpStream: TcpStream) {
    println!("Starting a game");
    let game = GameState::new(vec![tcpStream]);
}

pub fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    for stream in listener.incoming() {
        create_game(stream.unwrap());
    }
}