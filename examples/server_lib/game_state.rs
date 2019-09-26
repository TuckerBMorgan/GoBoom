use GoBooM::*;
use super::server_runes::*;
use super::controller::*;
use std::sync::mpsc::{Sender, Receiver};

pub struct GameState {
    pub board: GoBoard,
    pub turns: i32,
    runes: Vec<Box<dyn Rune>>,
    players: Vec<Box<dyn Controller>>,
    to_client_1: Sender<String>, 
    to_client_2: Sender<String>, 
    to_server: Receiver<String>
}

impl GameState {
    
    pub fn new(to_client_1: Sender<String>, 
                   to_client_2: Sender<String>, 
                   to_server: Receiver<String>) -> GameState {

        GameState {
            board: GoBoard::new(),
            to_client_1,
            to_client_2,
            to_server,
            turns: -1,
            players: vec![],
            runes: vec![]
        }

    }

    pub fn add_rune(&mut self, rune: Box<dyn Rune>) {
        self.runes.push(rune);
        self.execute_rune();
    }

    pub fn execute_rune(&mut self) {
        let rune = self.runes.remove(0);
        rune.execute(self);
        if self.runes.len() > 0 {
            self.execute_rune();
        }
    }

    pub fn new_controller(&mut self, controller: Box<dyn Controller>) {
        self.players.push(controller);
    }

    pub fn get_current_player_index(&self) -> usize {
        return self.turns as usize % 2;
    }

    pub fn run_game(mut self) {
        let new_game = NewGame::new();
        self.add_rune(Box::new(new_game));

        loop {
            let result = self.to_server.try_recv();
            match result {
                Ok(value) => {
                    //likely it is a options selection rune
                    //we should parse, verify, and execute
                },
                Err(_e) => {

                }
            }
        }
    }

    pub fn report_message_to_player(&mut self, message: String, player_index: usize) { 
        if player_index == 0 {
            let _ = self.to_client_1.send(message);
        }
        else if player_index == 1 {
            let _ = self.to_client_2.send(message);
        }
    }
}