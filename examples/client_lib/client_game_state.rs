use GoBooM::*;
use std::io::prelude::*;
use storm::*;
use std::str;
use storm::time::*;
use super::*;
use std::net::TcpStream;

use serde_json::{Value};

pub enum GameState {
    Waiting,
    PickingOption
}

pub struct ClientGameState {
    pub go_board: GoBoard,
    pub marked_index: i32,
    pub engine: Engine,
    pub sprites: Vec<Sprite>,
    pub screen: BatchToken,
    pub white_texture: Texture,
    pub black_texture: Texture,
    pub empty_area: Texture,
    pub clock: Clock,
    pub is_active: bool,
    pub stream: TcpStream,
    pub live_buffer : Vec<u8>,
    pub game_state: GameState
}

impl ClientGameState {
    pub fn new(go_board: GoBoard, mut engine: Engine, stream: TcpStream) -> ClientGameState {

        engine.window_clear_color(storm::color::RGBA8::new(0.392, 0.584, 0.929, 1.0));

        let white_texture = engine.texture_create(include_bytes!("../resources/images/WhitePiece.png").as_ref(), TextureFormat::PNG);
        let black_texture = engine.texture_create(include_bytes!("../resources/images/BlackPiece.png").as_ref(), TextureFormat::PNG);
        let empty_area = engine.texture_create(include_bytes!("../resources/images/Cross.png").as_ref(), TextureFormat::PNG);

        let screen = engine.batch_create(&BatchSettings::default());
        let mut sprites = Vec::new();

        let clock = Clock::new(144);

        let mut sprite = Sprite::default();
        sprite.texture = empty_area;
        sprite.size.x = 32;
        sprite.size.y = 32;

        for x in 0..19 {
            for y in 0..19 {
                sprite.pos.x = (32 * x) as f32 - 320.0;
                sprite.pos.y = (32 * y) as f32 - 256.0;
                sprites.push(sprite);
            }
        }

        engine.sprite_set(&screen, &sprites);

        ClientGameState {
            go_board,
            marked_index: -1,
            engine,
            sprites,
            screen,
            white_texture,
            black_texture,
            empty_area,
            clock,
            is_active: true,
            stream,
            live_buffer: vec![],
            game_state: GameState::Waiting
        }        
    }

    pub fn run(mut self) {
        while self.is_active {
            self.update();
            self.read_network_stream();
            self.render();
        }
    }

    pub fn update(&mut self) {
        while let Some(message) = self.engine.input_poll() {
            match message {
                InputMessage::CloseRequested => self.is_active = false,
                InputMessage::CursorMoved{pos, delta:_} => {
                    match self.game_state {
                        GameState::PickingOption => {
                            let x_index = ((pos.x  + 320.0) / 32.0) as i32;
                            let y_index = ((pos.y  + 256.0) / 32.0) as i32;

                            if x_index >= 0 && y_index >= 0 && x_index < 19 && y_index < 19 {
                                if self.go_board.board[x_index as usize][y_index as usize].status == TileStatus::Selectable {
                                    let index = x_index * 19 + y_index;
                                    self.marked_index = index;
                                }
                            }
                        },
                        _=> {

                        }
                    }
                },
                InputMessage::CursorPressed{button, pos} => {
                    match button {
                        CursorButton::Left => {
                            match self.game_state {
                                GameState::Waiting => {
                                    //Do Nothing
                                },
                                GameState::PickingOption => {
                                    let x_index = ((pos.x  + 320.0) / 32.0) as i32;
                                    let y_index = ((pos.y  + 256.0) / 32.0) as i32;

                                    if x_index >= 0 && y_index >= 0 && x_index < 19 && y_index < 19 {
                                        if self.go_board.board[x_index as usize][y_index as usize].status == TileStatus::Selectable {
                                            let pick_option = PickOption::new(x_index as usize, y_index as usize);
                                            pick_option.execute(self);
                                        }
                                    }
                                }
                            }
                        },
                        _=> {

                        }
                    }
                },
                _ => {

                }
            }
        }
    }

    pub fn report_message_to_server(&mut self, message: String) {
        let _ = self.stream.write(message.as_bytes());
        let _ = self.stream.write("@@".as_bytes());
    }

    pub fn read_network_stream(&mut self) {
        let mut read_buffer = [0;128];
        let read_bytes = self.stream.read(&mut read_buffer);

        match read_bytes {
            Ok(number) => {
                if number > 0 {
                    let mut new_vec = read_buffer[0..number].to_vec();
                    self.live_buffer.append(&mut new_vec);
                    
                    let mut has_found_first_part = false;
                    for i in 0..self.live_buffer.len() {
                        if self.live_buffer[i] as char == '@' {
                            if has_found_first_part == false {
                                has_found_first_part = true;
                            }
                            else {
                                let use_values : Vec<u8> = self.live_buffer.drain(0..i+1).collect();
                                let result = str::from_utf8(&use_values[0..i-1]).unwrap();
                                self.handle_rune(String::from(result));
                                break;
                            }
                        }
                    }
                }
            },
            Err(_e) => {
                //println!("{:?}", e);
            }
        }
    }

    fn handle_rune(&mut self, message: String) {
        println!("{:?}", message);
        let v : Value = serde_json::from_str(&message).unwrap();
        let as_obj = v.as_object().unwrap();
        let rune_type = as_obj.get("rune_type").unwrap().as_str().unwrap();

        if rune_type == "set_board_state" {
            let sbs : SetBoardState = serde_json::from_str(&message).unwrap();
            sbs.execute(self);
        }
        else if rune_type == "new_game" {
            let ng : NewGame = serde_json::from_str(&message).unwrap();
            ng.execute(self);
        }
        else if rune_type == "new_controller" {
            let nc : NewController = serde_json::from_str(&message).unwrap();
            nc.execute(self);
        }
        else if rune_type == "report_options" {
            let ro : ReportOptionsRune = serde_json::from_str(&message).unwrap();
            ro.execute(self);
        }
    }

    pub fn render(&mut self) {
        for x in 0..19 {
            for y in 0..19 {
                let index = x * 19 + y;
                if index == self.marked_index {
                    self.sprites[index as usize].color = color::RED;
                }
                else {
                    let index = index as usize;
                    match self.go_board.board[x as usize][y as usize].status {
                        TileStatus::Empty => {
                            self.sprites[index].color = color::WHITE;
                            self.sprites[index].texture = self.empty_area;
                        },
                        TileStatus::White => {
                            self.sprites[index].color = color::WHITE;
                            self.sprites[index].texture = self.white_texture;
                        },
                        TileStatus::Black => {
                            self.sprites[index].color = color::WHITE;
                            self.sprites[index].texture = self.black_texture;
                        },
                        TileStatus::Selectable => {
                            self.sprites[index].texture = self.white_texture;
                            self.sprites[index].color = color::GREEN;
                        }
                    }
                }
            }
        }

        self.engine.sprite_set(&self.screen, &self.sprites);
        self.engine.window_commit();
        self.clock.tick();
    }
}