```rust
use super::Piece;
use crate::chess::board::{Board, Move};
use crate::chess::utils::Position;

pub struct Rook {
    position: Position,
    has_moved: bool,
}

impl Rook {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            has_moved: false,
        }
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}

impl Piece for Rook {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn is_valid_move(&self, board: &Board, m: &Move) -> bool {
        // Implement the logic for valid rook move
    }
}
```