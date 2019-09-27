use GoBooM::*;

//represents who can play a game
pub struct Controller {
    pub color: TileStatus
}

impl Controller {
    pub fn new(color: TileStatus) -> Controller {
        Controller {
            color
        }
    }
}