```rust
use crate::chess::piece::Piece;
use crate::chess::board::Board;
use crate::chess::game::Game;

pub struct Bishop {
    pub position: (usize, usize),
    pub color: String,
}

impl Bishop {
    pub fn new(position: (usize, usize), color: String) -> Bishop {
        Bishop { position, color }
    }

    pub fn is_valid_move(&self, game: &Game, new_position: (usize, usize)) -> bool {
        let dx = (self.position.0 as i32 - new_position.0 as i32).abs();
        let dy = (self.position.1 as i32 - new_position.1 as i32).abs();

        if dx == dy {
            let direction = (
                (new_position.0 as i32 - self.position.0 as i32) / dx,
                (new_position.1 as i32 - self.position.1 as i32) / dy,
            );

            let mut current_position = self.position;

            while current_position != new_position {
                current_position = (
                    (current_position.0 as i32 + direction.0) as usize,
                    (current_position.1 as i32 + direction.1) as usize,
                );

                if let Some(piece) = game.board.get_piece(current_position) {
                    if piece.color == self.color {
                        return false;
                    }
                }
            }

            return true;
        }

        false
    }

    pub fn get_possible_moves(&self, game: &Game) -> Vec<(usize, usize)> {
        let mut possible_moves = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx.abs() == dy.abs() {
                    let mut current_position = self.position;

                    loop {
                        current_position = (
                            (current_position.0 as i32 + dx) as usize,
                            (current_position.1 as i32 + dy) as usize,
                        );

                        if !game.board.is_valid_position(current_position) {
                            break;
                        }

                        if let Some(piece) = game.board.get_piece(current_position) {
                            if piece.color == self.color {
                                break;
                            }

                            possible_moves.push(current_position);

                            if piece.color != self.color {
                                break;
                            }
                        } else {
                            possible_moves.push(current_position);
                        }
                    }
                }
            }
        }

        possible_moves
    }
}
```