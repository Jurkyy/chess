```rust
use super::mod::Piece;
use crate::chess::board::Board;
use crate::chess::utils::Position;

pub struct Knight {
    position: Position,
    color: String,
}

impl Piece for Knight {
    fn new(position: Position, color: String) -> Self {
        Knight { position, color }
    }

    fn is_valid_move(&self, board: &Board, new_position: Position) -> bool {
        let possible_moves = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];

        for move_ in possible_moves.iter() {
            let potential_position = Position {
                x: self.position.x + move_.0,
                y: self.position.y + move_.1,
            };

            if potential_position == new_position {
                if let Some(piece) = board.get_piece_at_position(&potential_position) {
                    if piece.get_color() != self.color {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }

        false
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_color(&self) -> &str {
        &self.color
    }

    fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
    }
}
```