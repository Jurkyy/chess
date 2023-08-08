```rust
use super::mod::Piece;
use crate::chess::board::Board;
use crate::chess::utils::Position;

pub struct Pawn {
    position: Position,
    color: String,
}

impl Piece for Pawn {
    fn new(position: Position, color: String) -> Self {
        Pawn { position, color }
    }

    fn is_valid_move(&self, board: &Board, new_position: Position) -> bool {
        let dx = (self.position.x - new_position.x).abs();
        let dy = (self.position.y - new_position.y).abs();

        if self.color == "white" {
            dy == 1 && dx == 0 && new_position.y > self.position.y
        } else {
            dy == 1 && dx == 0 && new_position.y < self.position.y
        }
    }

    fn move_piece(&mut self, new_position: Position) {
        self.position = new_position;
    }
}
```