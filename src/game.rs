use crate::player::Player;

pub struct Game {
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player1: Player::new(),
            player2: Player::new(),
        }
    }
}
