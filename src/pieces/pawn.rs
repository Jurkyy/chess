```rust
use crate::board::{Board, Move};
use crate::pieces::piece::Piece;

pub struct Pawn {
    pub color: bool,
    pub has_moved: bool,
}

impl Piece for Pawn {
    fn new(color: bool) -> Self {
        Pawn {
            color,
            has_moved: false,
        }
    }

    fn get_color(&self) -> bool {
        self.color
    }

    fn get_valid_moves(&self, board: &Board, x: usize, y: usize) -> Vec<Move> {
        let mut moves = Vec::new();

        let direction = if self.color { -1 } else { 1 };

        if board.is_valid_move(x, y, x, y as isize + direction) {
            moves.push(Move::new(x, y, x, y + direction as usize));
        }

        if !self.has_moved && board.is_valid_move(x, y, x, y as isize + 2 * direction) {
            moves.push(Move::new(x, y, x, y + 2 * direction as usize));
        }

        if board.is_valid_capture(x, y, x - 1, y as isize + direction) {
            moves.push(Move::new(x, y, x - 1, y + direction as usize));
        }

        if board.is_valid_capture(x, y, x + 1, y as isize + direction) {
            moves.push(Move::new(x, y, x + 1, y + direction as usize));
        }

        moves
    }

    fn after_move(&mut self) {
        self.has_moved = true;
    }
}
```