use serde::{Serialize, Deserialize};
use GoBooM::*;

pub trait Rune : Send {
    fn execute();
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SetBoardState {
   pub  board: [[u8;19];19]
}

impl SetBoardState {
    /*
    pub fn new(go_board: &GoBoard) -> SetBoardState {
        let mut board  : [[u8; 19]; 19] = [[0;19];19];

        for x in 0..19 {
            for y in 0..19 {
                board[x][y] = 0;
            }
        }

        SetBoardState {
            board
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"set_board_state\",")
    }
    */
}

impl Rune for SetBoardState {
    fn execute(){ 

    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ReportOptionsRune {
   pub  board: [[bool;19];19]
}

impl ReportOptionsRune {
    /*
    pub fn new(go_board: &GoBoard) -> ReportOptionsRune {
        let mut board  : [[bool; 19]; 19] = [[false;19];19];

        for x in 0..19 {
            for y in 0..19 {
                match go_board.board[x][y].status {
                    TileStatus::Empty => {
                        board[x][y] = true;
                    },
                    _ => {
                        board[x][y] = false;
                    }
                }
            }
        }

        ReportOptionsRune {
            board
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"report_options\",")
    }
    */
}

impl Rune for ReportOptionsRune {
    fn execute() {

    }
}