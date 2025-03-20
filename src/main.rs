mod ship_type;
mod player;
mod game;

use ship_type::ShipType;
use game::Game;

fn main() {
    let mut game = Game::new();

    // Exemple de placement de navires pour le joueur 1
    let _ = game.player1.place_ship("B4", ShipType::Carrier, 'E');
    let _ = game.player1.place_ship("C8", ShipType::Battleship, 'E');
    let _ = game.player1.place_ship("E1", ShipType::Cruiser, 'S');
    let _ = game.player1.place_ship("G3", ShipType::Submarine, 'E');
    let _ = game.player1.place_ship("J5", ShipType::Destroyer, 'S'); // Devrait déclencher le message "Ready"

    println!("\n--- Plateau du joueur 1 ---");
    game.player1.print_board();

    println!("\n--- Plateau du joueur 2 ---");
    game.player1.draw_board_for_opponent();
}
