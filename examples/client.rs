use GoBooM;
use storm::*;
use storm::time::*;
use std::net::TcpStream;

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

fn game(mut engine: Engine) {
    let stream = connect_to_server();
    //return;
    engine.window_clear_color(storm::color::RGBA8::new(0.392, 0.584, 0.929, 1.0));

    let white_texture = engine.texture_create(include_bytes!("resources/images/WhitePiece.png").as_ref(), TextureFormat::PNG);
    let black_texture = engine.texture_create(include_bytes!("resources/images/BlackPiece.png").as_ref(), TextureFormat::PNG);
    

    let mut is_active = true;
    let screen = engine.batch_create(&BatchSettings::default());
    let mut sprites = Vec::new();

    let mut clock = Clock::new(144);


    let mut sprite = Sprite::default();
    sprite.texture = white_texture;
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

    while is_active {

        while let Some(message) = engine.input_poll() {
            match message {
                InputMessage::CloseRequested => is_active = false,
                _ => {

                }
            }
        }
        engine.window_commit();
        clock.tick();
    }
}