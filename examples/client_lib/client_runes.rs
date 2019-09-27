use serde::{Serialize, Deserialize};
use GoBooM::*;
use super::*;

pub trait Rune : Send {
    fn execute(&self, client_state: &mut ClientGameState);
    fn to_string(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SetBoardState {
   pub  board: [[u8;19];19]
}

impl Rune for SetBoardState {
    fn execute(&self, client_state: &mut ClientGameState) {
        println!("Set Board State Rune");

          for x in 0..19 {
            for y in 0..19 {
                match self.board[x][y] {
                    0 => {
                        client_state.go_board.board[x][y].status = TileStatus::Empty;
                    },
                    1 => {
                        client_state.go_board.board[x][y].status = TileStatus::White;
                    },
                    2 => {
                        client_state.go_board.board[x][y].status = TileStatus::Black;
                    },
                    _=> {
                        println!("This means that a data has yet to work");
                    }
                }
            }
        }

    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"set_board_state\",")
    }

}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ReportOptionsRune {
   pub  board: [[bool;19];19]
}

impl Rune for ReportOptionsRune {
    fn execute(&self, client_state: &mut ClientGameState) {
        println!("Report Options Rune");
        client_state.game_state = GameState::PickingOption;

        for x in 0..19 {
            for y in 0..19 {
                if self.board[x][y] {
                    client_state.go_board.board[x][y].status = TileStatus::Selectable;
                }
            }
        }
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"report_options\",")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NewGame {

}


impl Rune for NewGame {

    fn execute(&self, _: &mut ClientGameState) {
        println!("New Game Rune");
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"new_game\"")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NewController {
    is_me: bool,
    id: usize,
    color: TileStatus
}

impl Rune for NewController {
    fn execute(&self, _: &mut ClientGameState) {
        println!("New Controller");
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"new_controller\",")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct RotateTurn {}

impl Rune for RotateTurn {
   fn execute(&self, _: &mut ClientGameState) {
        println!("Rotate Turn");
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"rotate_turn\"")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PickOption {
    x: usize,
    y: usize
}

impl PickOption {
    pub fn new(x: usize, y: usize) -> PickOption {
        PickOption {
            x,
            y
        }
    }
}

impl Rune for PickOption {
   fn execute(&self, cgs: &mut ClientGameState) {
       cgs.report_message_to_server(self.to_string());

        cgs.marked_index = -1;
        cgs.game_state = GameState::Waiting;

       for x in 0..19 {
           for y in 0..19 {
               match cgs.go_board.board[x][y].status {
                    TileStatus::Selectable => {
                        cgs.go_board.board[x][y].status = TileStatus::Empty;
                    },
                    _ => {

                    }
               }
           }
       }
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"pick_option\",")
    }
}