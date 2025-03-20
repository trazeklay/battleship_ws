use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::ship_type::ShipType;

pub const GRID_SIZE: usize = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub uuid: Option<String>,
    pub board: [[u8; GRID_SIZE]; GRID_SIZE],
    pub placed_ships: HashSet<ShipType>, // Liste des bateaux placés
    pub is_ready: bool, // Indique si tous les bateaux sont placés
}

impl Player {
    pub fn new() -> Self {
        Self {
            uuid: None,
            board: [[0; GRID_SIZE]; GRID_SIZE],
            placed_ships: HashSet::new(),
            is_ready: false,
        }
    }

    /// Convertit une position sous forme de "A6" en coordonnées (x, y)
    pub fn parse_position(pos: &str) -> Result<(usize, usize), String> {
        if pos.len() < 2 {
            return Err("Format invalide, utilisez une lettre suivie d'un chiffre (ex: A6)".to_string());
        }

        let mut chars = pos.chars();
        let letter = chars.next().unwrap().to_ascii_uppercase();
        let number: String = chars.collect();

        let x = letter as usize - 'A' as usize;
        let y = number.parse::<usize>().map_err(|_| "Numéro invalide".to_string())?;

        if x >= GRID_SIZE || y == 0 || y > GRID_SIZE {
            return Err("Position hors du plateau".to_string());
        }

        Ok((x, y - 1))
    }

    /// Place un navire sur le plateau et vérifie si tous les bateaux sont placés.
    /// Affiche également la grille après un placement réussi.
    pub fn place_ship(&mut self, start_pos: &str, ship_type: ShipType, direction: char) -> Result<(), String> {
        if self.is_ready {
            return Err("Tous les navires ont déjà été placés, le joueur est prêt !".to_string());
        }

        if self.placed_ships.contains(&ship_type) {
            return Err(format!("Le {:?} a déjà été placé !", ship_type));
        }

        let (x, y) = Self::parse_position(start_pos)?;
        let length = ship_type.size();

        let (dx, dy) = match direction {
            'N' => (0, -1),
            'E' => (1, 0),
            'S' => (0, 1),
            'W' => (-1, 0),
            _ => return Err("Direction invalide. Utilisez 'N', 'E', 'S' ou 'W'.".to_string()),
        };

        let mut ship_cells = Vec::new();

        for i in 0..length {
            let new_x = x as isize + i as isize * dx;
            let new_y = y as isize + i as isize * dy;

            if new_x < 0 || new_x >= GRID_SIZE as isize || new_y < 0 || new_y >= GRID_SIZE as isize {
                return Err("Placement hors limites !".to_string());
            }

            let ux = new_x as usize;
            let uy = new_y as usize;

            if self.board[uy][ux] & 0b01 != 0 {
                return Err("Chevauchement avec un autre navire !".to_string());
            }

            ship_cells.push((ux, uy));
        }

        for (ux, uy) in ship_cells {
            self.board[uy][ux] |= 0b01;
        }

        // Ajouter le bateau à la liste des bateaux placés
        self.placed_ships.insert(ship_type);

        // Vérifier si tous les bateaux ont été placés
        if self.placed_ships.len() == 5 {
            self.is_ready = true;
            println!("✅ Tous les navires ont été placés ! Le joueur est prêt !");
        }

        // Dessiner la grille après un placement réussi
        self.print_board();

        Ok(())
    }

    /// Affiche le plateau complet (pour le joueur)
    pub fn print_board(&self) {
        print!("    ");
        for col in 0..GRID_SIZE {
            print!("  {} ", (b'A' + col as u8) as char);
        }
        println!();

        for row in 0..GRID_SIZE {
            print!("    ");
            for _ in 0..GRID_SIZE {
                print!("+---");
            }
            println!("+");

            print!("{:2}  ", row + 1);
            for col in 0..GRID_SIZE {
                let cell = self.board[row][col];
                let symbol = match cell {
                    0 => ' ',  // Eau
                    1 => '□',  // Navire intact
                    2 => '○',  // Tir manqué
                    3 => '▣',  // Navire touché
                    _ => '?',
                };
                print!("| {} ", symbol);
            }
            println!("|");
        }

        print!("    ");
        for _ in 0..GRID_SIZE {
            print!("+---");
        }
        println!("+");
    }

    /// Affiche la grille telle qu'elle serait vue par l'adversaire.
    /// Seules les cases où un tir a été effectué (2 ou 3) sont affichées.
    pub fn draw_board_for_opponent(&self) {
        print!("    ");
        for col in 0..GRID_SIZE {
            print!("  {} ", (b'A' + col as u8) as char);
        }
        println!();

        for row in 0..GRID_SIZE {
            print!("    ");
            for _ in 0..GRID_SIZE {
                print!("+---");
            }
            println!("+");

            print!("{:2}  ", row + 1);
            for col in 0..GRID_SIZE {
                let cell = self.board[row][col];
                let symbol = match cell {
                    2 => '○',  // Tir manqué
                    3 => '▣',  // Tir réussi (bateau touché)
                    _ => ' ',   // Position non ciblée
                };
                print!("| {} ", symbol);
            }
            println!("|");
        }

        print!("    ");
        for _ in 0..GRID_SIZE {
            print!("+---");
        }
        println!("+");
    }
}
