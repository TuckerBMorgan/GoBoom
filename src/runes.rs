use serde::{Serialize, Deserialize};


use super::GoBoard;


pub trait Rune {
    fn to_string(&self) -> String;
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SetBoardState {
    board: [[u8;19];19]
}

impl SetBoardState {
    pub fn new(go_board: &GoBoard) -> SetBoardState {
        let mut board  = [[0;19];19];

        for x in 0..19 {
            for y in 0..19 {
                board[x][y] = go_board.board[x][y].status.into_u8();
            }
        }

        SetBoardState {
            board
        }
    }
}

impl Rune for SetBoardState {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}