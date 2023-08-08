```rust
use super::mod::Piece;
use crate::chess::board::Board;
use crate::chess::utils::Position;

pub struct Queen {
    position: Position,
    color: bool, // true for white, false for black
}

impl Piece for Queen {
    fn new(position: Position, color: bool) -> Self {
        Queen { position, color }
    }

    fn is_valid_move(&self, board: &Board, new_position: Position) -> bool {
        let dx = (self.position.x - new_position.x).abs();
        let dy = (self.position.y - new_position.y).abs();

        // Queen can move in any direction, any number of squares
        dx == dy || self.position.x == new_position.x || self.position.y == new_position.y
    }
}
```