mod client_lib;
//use GoBooM;
use std::io::prelude::*;
use storm::*;
use std::str;
use storm::time::*;
use std::net::TcpStream;

use GoBooM::*;
use client_lib::*;

use serde_json::{Result, Value};

fn main() {
    Engine::start(
        WindowSettings {
            title: String::from("Storm: GoBoom"),
            display_mode: DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            },
            vsync: Vsync::Disabled,
        },
        game,
    );
}
 

fn connect_to_server() -> Option<TcpStream> {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            return Some(stream);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    None
}

fn handle_rune(message: String, go_board: &mut GoBoard) {
    println!("{:?}", message);
    let v : Value = serde_json::from_str(&message).unwrap();
    let as_obj = v.as_object().unwrap();
    let rune_type = as_obj.get("rune_type").unwrap().as_str().unwrap();

    if rune_type == "set_board_state" {
        let sbs : SetBoardState = serde_json::from_str(&message).unwrap();
        for x in 0..19 {
            for y in 0..19 {
                match sbs.board[x][y] {
                    0 => {
                        go_board.board[x][y].status = TileStatus::Empty;
                    },
                    1 => {
                        go_board.board[x][y].status = TileStatus::White;
                    },
                    2 => {
                        go_board.board[x][y].status = TileStatus::Black;
                    },
                    _=> {
                        println!("This means that a data has yet to work");
                    }
                }
            }
        }
    }
    else {

    }
}

fn game(mut engine: Engine) {
    let mut stream = connect_to_server();
    let mut stream = stream.unwrap();

    //return;
    engine.window_clear_color(storm::color::RGBA8::new(0.392, 0.584, 0.929, 1.0));

    let white_texture = engine.texture_create(include_bytes!("resources/images/WhitePiece.png").as_ref(), TextureFormat::PNG);
    let black_texture = engine.texture_create(include_bytes!("resources/images/BlackPiece.png").as_ref(), TextureFormat::PNG);
    let empty_area = engine.texture_create(include_bytes!("resources/images/Cross.png").as_ref(), TextureFormat::PNG);

    let mut is_active = true;
    let screen = engine.batch_create(&BatchSettings::default());
    let mut sprites = Vec::new();

    let mut clock = Clock::new(144);

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

    let mut read_buffer = [0;128];

    let mut live_buffer : Vec<u8> = vec![];
    let _ = stream.set_nonblocking(true);

    let mut go_boom = GoBoard::new();

    while is_active {

        let read_bytes = stream.read(&mut read_buffer);

        match read_bytes {
            Ok(number) => {
                if number > 0 {
                    let mut new_vec = read_buffer[0..number].to_vec();
                    live_buffer.append(&mut new_vec);
                    
                    let mut has_found_first_part = false;
                    for i in 0..live_buffer.len() {
                        if live_buffer[i] as char == '@' {
                            if has_found_first_part == false {
                                has_found_first_part = true;
                            }
                            else {
                                let use_values : Vec<u8> = live_buffer.drain(0..i+1).collect();
                                let result = str::from_utf8(&use_values[0..i-1]).unwrap();
                                handle_rune(String::from(result), &mut go_boom);
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
        while let Some(message) = engine.input_poll() {

            match message {
                InputMessage::CloseRequested => is_active = false,
                _ => {

                }
            }
        }

        for x in 0..19 {
            for y in 0..19 {
                let index = x * 19 + y;
                match go_boom.board[x][y].status {
                    TileStatus::Empty => {
                        sprites[index].texture = empty_area;
                    },
                    TileStatus::White => {
                        sprites[index].texture = white_texture;
                    },
                    TileStatus::Black => {
                        sprites[index].texture = black_texture;
                    }
                }
            }
        }

        engine.sprite_set(&screen, &sprites);
        engine.window_commit();
        clock.tick();
    }
}