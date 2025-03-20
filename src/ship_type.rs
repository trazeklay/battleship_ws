use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub enum ShipType {
    Carrier,      // Taille 5
    Battleship,   // Taille 4
    Cruiser,      // Taille 3
    Submarine,    // Taille 3
    Destroyer,    // Taille 2
}

impl ShipType {
    pub fn size(&self) -> usize {
        match self {
            ShipType::Carrier => 5,
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::Submarine => 3,
            ShipType::Destroyer => 2,
        }
    }
}
