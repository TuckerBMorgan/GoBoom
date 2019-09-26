use std::net::TcpStream;
use GoBooM::*;
use std::io::prelude::*;

//represents who can play a game
pub trait Controller {
    fn send_message(&mut self, message: String);
}

pub struct PlayerController {
   // tcp_stream: TcpStream
}

impl PlayerController {
    pub fn new() -> PlayerController {
        PlayerController {
        }
    }

    pub fn run() {

    }
}

impl Controller for PlayerController {
    fn send_message(&mut self, message: String) {
  //      let _ = self.tcp_stream.write(&message.to_string().into_bytes());
 //       let _ = self.tcp_stream.write(b"@@");
    }
}

pub struct AIController {
    board: GoBoard
}

impl AIController {
    pub fn new() -> AIController {
        AIController {
            board: GoBoard::new()
        }
    }
}

impl Controller for AIController {
    fn send_message(&mut self, message: String) {
        
    }
}