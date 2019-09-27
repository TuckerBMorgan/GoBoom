use serde::{Serialize, Deserialize};

pub struct Pos {
    x: usize,
    y: usize
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TileStatus {
    Empty,
    Selectable,
    White,
    Black
}

impl TileStatus {
    pub fn display(&self) -> char {
        match self {
            TileStatus::White => {
                '0'
            },
            TileStatus::Selectable => {
                '+'
            },
            TileStatus::Black => {
                '='
            },
            TileStatus::Empty => {
                '_'
            }
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            TileStatus::Empty => {
                0
            },
            TileStatus::White => {
                1
            },
            TileStatus::Black => {
                2
            },
            TileStatus::Selectable => {
                3
            }

        }
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub status: TileStatus,
    pub is_alive: bool
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            status: TileStatus::Empty,
            is_alive: false
        }
    }
}



pub struct GoBoard {
    pub board: [[Tile;20];20]    
}

impl GoBoard {

    pub fn new() -> GoBoard {
        GoBoard {
            board: [[Tile::new();20];20]
        }    
    }

    pub fn grant_life(&mut self, x: usize, y: usize, color: TileStatus) {
        self.board[x][y].is_alive = true;

        let neb = GoBoard::generate_good_indexes(x, y);
        for n in neb {
            if self.board[n.x][n.y].status == color && self.board[n.x][n.y].is_alive == false {
                self.grant_life(n.x, n.y, color)
            }
        }
    }

    pub fn generate_good_indexes(x: usize, y: usize) -> Vec<Pos> {
        let i_x = x as isize;
        let i_y = y as isize;

        let possible_pos = vec![
            (i_x - 1, i_y),
            (i_x, i_y - 1),
            (i_x + 1, i_y),
            (i_x, i_y + 1),
        ];

        return possible_pos.iter().filter(|val| val.0 >= 0 && val.1 >= 0 && val.0 < 20 && val.1 < 20 )
                                .map(|val|Pos{x: val.0 as usize, y: val.1 as usize}).collect();
    }

    pub fn resolve_board(&mut self) {
        for x in 0..20 {
            for y in 0..20 {
                self.board[x][y].is_alive = false;
            }
        }
        
        for x in 0..20 {
            for y in 0..20 {
                if self.board[x][y].status == TileStatus::Empty {
                    let neb = GoBoard::generate_good_indexes(x, y);
                    for n in neb {
                        if self.board[n.x][n.y].is_alive == false {
                            self.grant_life(n.x, n.y, self.board[n.x][n.y].status);
                        }
                    }
                }
            }
        }

        for x in 0..20 {
            for y in 0..20 {
                if self.board[x][y].is_alive == false && self.board[x][y].status != TileStatus::Empty {
                    self.board[x][y].status = TileStatus::Empty;
                }
            }
        }
        

    }

    pub fn print_board(&self) {
        for x in 0..20 {
            for y in 0..20 {
                print!("{}", self.board[x][y].status.display());
            }

            println!("");
        }
    }
}