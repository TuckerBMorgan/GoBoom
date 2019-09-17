use std::net::TcpStream;

//represents who can play a game
pub trait Controller {
    
}

pub struct PlayerController {
    tcpStream: TcpStream
}

impl PlayerController {
    pub fn new(tcpStream: TcpStream) -> PlayerController {
        PlayerController {
            tcpStream
        }
    }
}

impl Controller for PlayerController {

}

pub struct AIController {

}

impl AIController {
    pub fn new() -> AIController {
        AIController {

        }
    }
}

impl Controller for AIController {

}