use serde::{Serialize, Deserialize};
use GoBooM::*;
use super::*;

pub trait Rune : Send {
    fn execute(&self, game_state: &mut GameState);
    fn to_string(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SetBoardState {
   pub  board: [[u8;19];19]
}

impl SetBoardState {
    pub fn new(_: &GoBoard) -> SetBoardState {
        let mut board  : [[u8; 19]; 19] = [[0;19];19];

        for x in 0..19 {
            for y in 0..19 {
                board[x][y] = 0;//(x % 3) as u8;
            }
        }

        SetBoardState {
            board
        }
    }

}

impl Rune for SetBoardState {
    fn execute(&self, game_state: &mut GameState) {
        println!("Set Board State Rune");
        for x in 0..19 {
            for y in 0..19 {
                game_state.board.board[x][y].status = TileStatus::Empty;
            }
        }

        game_state.report_message_to_player(self.to_string(), 0);
        game_state.report_message_to_player(self.to_string(), 1);
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"set_board_state\",")
    }

}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ReportOptionsRune {
   pub  board: [[bool;19];19]
}

impl ReportOptionsRune {
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

}


impl Rune for ReportOptionsRune {
    fn execute(&self, game_state: &mut GameState) {
        println!("Report Options Rune");

        game_state.report_message_to_player(self.to_string(), game_state.get_current_player_index());
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"report_options\",")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NewGame {

}

impl NewGame {
    pub fn new() -> NewGame {
        NewGame {

        }
    }
}

impl Rune for NewGame {

    fn execute(&self, game_state: &mut GameState) {
        println!("New Game");
        //new game runes to players
        game_state.report_message_to_player(self.to_string(), 0);
        game_state.report_message_to_player(self.to_string(), 1);

        let first_new_controller = NewController::new(false, 0);
        let second_new_controller = NewController::new(false, 1);

        game_state.add_rune(Box::new(first_new_controller));
        game_state.add_rune(Box::new(second_new_controller));

        //set board state rune
        let set_board = SetBoardState::new(&game_state.board);

        game_state.add_rune(Box::new(set_board));
        
        //rotate turn rune

        let rotate_turn = RotateTurn::new();
        game_state.add_rune(Box::new(rotate_turn));

        //send options rune
        let send_options_runes = ReportOptionsRune::new(&game_state.board);
        game_state.add_rune(Box::new(send_options_runes));

    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"new_game\"")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NewController {
    is_me: bool,
    id: usize
}

impl NewController {
    pub fn new(is_me: bool, id: usize) -> NewController {
        NewController {
            is_me,
            id
        }
    }
}

impl Rune for NewController {
    fn execute(&self, game_state: &mut GameState) {
        println!("New Controller");

        let mut clones = self.clone();
        if clones.id == 0 {
            game_state.report_message_to_player(clones.to_string(), 1);
            clones.is_me = true;
            game_state.report_message_to_player(clones.to_string(), 0);
        }
        else if clones.id == 1 {
            game_state.report_message_to_player(clones.to_string(), 0);
            clones.is_me = true;
            game_state.report_message_to_player(clones.to_string(), 1);
        }

        game_state.new_controller(Box::new(PlayerController::new()));
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"new_controller\",")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct RotateTurn {}

impl RotateTurn {pub fn new() -> RotateTurn {RotateTurn {}}}

impl Rune for RotateTurn {
   fn execute(&self, game_state: &mut GameState) {
        println!("Rotate Turn");

        game_state.turns += 1;
        game_state.report_message_to_player(self.to_string(), 0);
        game_state.report_message_to_player(self.to_string(), 1);
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"rune_type\":\"rotate_turn\"")
    }
}