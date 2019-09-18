use GoBooM::*;
use super::controller::*;
use std::net::TcpStream;
use serde::{Serialize, Deserialize};

pub struct GameState {
    board: GoBoard,
    players: Vec<Box<dyn Controller>>
}

impl GameState {
    pub fn new(mut tcp_streams: Vec<TcpStream>) -> GameState {

        let mut players : Vec<Box<dyn Controller>> = vec![];

        //this means that the player wants an AI Game
        if tcp_streams.len() == 1 {
            let player = PlayerController::new(tcp_streams.remove(0));
            let ai_player = AIController::new();
            players.push(Box::new(player));
            players.push(Box::new(ai_player));
        }

        GameState {
            board: GoBoard::new(),
            players
        }
    }

    pub fn run_game(mut self) {
        let setBoardStateRune = SetBoardState::new(&self.board);
        for player in self.players.iter_mut() {
            player.send_message(Box::new(setBoardStateRune));
        }
        loop {

        }
    }
}