use uuid::Uuid;

use crate::ship_type::ShipType;

const GRID_SIZE: u8 = 10;

pub struct Player {
    uuid: String,
    grid: [[u8; GRID_SIZE]; GRID_SIZE]
}

impl Player {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            grid: [[0; GRID_SIZE]; GRID_SIZE]
        }
    }
}