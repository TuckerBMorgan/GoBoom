use GoBooM::GoBoard;
use super::controller::*;
use std::net::TcpStream;

pub struct GameState {
    board: GoBoard,
    players: Vec<Box<dyn Controller>>
}

impl GameState {
    pub fn new(mut tcpStreams: Vec<TcpStream>) -> GameState {

        let mut players : Vec<Box<Controller>> = vec![];

        //this means that the player wants an AI Game
        if tcpStreams.len() == 1 {
            let player = PlayerController::new(tcpStreams.remove(0));
            let ai_player = AIController::new();
            players.push(Box::new(player));
            players.push(Box::new(ai_player));
        }

        GameState {
            board: GoBoard::new(),
            players
        }
    }
}