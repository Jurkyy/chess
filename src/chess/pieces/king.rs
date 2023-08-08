```rust
use super::Piece;
use crate::chess::board::{Board, Move};
use crate::chess::utils::Position;

pub struct King {
    position: Position,
    has_moved: bool,
}

impl King {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            has_moved: false,
        }
    }

    pub fn can_castle(&self, board: &Board, rook_position: Position) -> bool {
        if self.has_moved {
            return false;
        }

        let rook = board.get_piece(rook_position);
        if let Some(rook) = rook {
            if rook.has_moved() {
                return false;
            }
        } else {
            return false;
        }

        let (start, end) = if self.position.x < rook_position.x {
            (self.position.x, rook_position.x)
        } else {
            (rook_position.x, self.position.x)
        };

        for x in (start + 1)..end {
            if board.get_piece(Position { x, y: self.position.y }).is_some() {
                return false;
            }
        }

        true
    }
}

impl Piece for King {
    fn position(&self) -> Position {
        self.position
    }

    fn move_piece(&mut self, new_position: Position) {
        self.position = new_position;
        self.has_moved = true;
    }

    fn is_valid_move(&self, board: &Board, new_position: Position) -> bool {
        let dx = (self.position.x as i32 - new_position.x as i32).abs();
        let dy = (self.position.y as i32 - new_position.y as i32).abs();

        dx <= 1 && dy <= 1
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }
}
```