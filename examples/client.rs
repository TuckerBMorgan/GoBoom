mod client_lib;
//use GoBooM;
use storm::*;
use std::net::TcpStream;

use GoBooM::*;
use client_lib::*;


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
        Ok(stream) => {
            return Some(stream);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    None
}




fn game(engine: Engine) {
    let stream = connect_to_server();
    let stream = stream.unwrap();

    let _ = stream.set_nonblocking(true);

    let go_boom = GoBoard::new();

    let client_state = ClientGameState::new(go_boom, engine, stream);

    client_state.run();
}