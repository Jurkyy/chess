```rust
use super::board::Board;
use super::piece::Piece;
use super::game::Game;

pub struct Knight {
    pub position: (usize, usize),
    pub color: String,
}

impl Knight {
    pub fn new(position: (usize, usize), color: String) -> Knight {
        Knight { position, color }
    }

    pub fn is_valid_move(&self, game: &Game, new_position: (usize, usize)) -> bool {
        let dx = (self.position.0 as i32 - new_position.0 as i32).abs();
        let dy = (self.position.1 as i32 - new_position.1 as i32).abs();

        if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
            return true;
        }

        false
    }

    pub fn move_piece(&mut self, game: &mut Game, new_position: (usize, usize)) {
        if self.is_valid_move(game, new_position) {
            self.position = new_position;
        } else {
            panic!("InvalidMoveError");
        }
    }

    pub fn get_possible_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let mut possible_moves = Vec::new();

        for dx in -2..=2 {
            for dy in -2..=2 {
                if dx.abs() + dy.abs() == 3 {
                    let new_position = (self.position.0 as i32 + dx, self.position.1 as i32 + dy);
                    if new_position.0 >= 0 && new_position.0 < BOARD_SIZE as i32 && new_position.1 >= 0 && new_position.1 < BOARD_SIZE as i32 {
                        possible_moves.push((new_position.0 as usize, new_position.1 as usize));
                    }
                }
            }
        }

        possible_moves
    }
}
```