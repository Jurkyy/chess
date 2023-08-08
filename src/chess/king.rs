```rust
use super::piece::Piece;
use super::color::Color;
use super::board::Board;
use super::move::Move;

pub struct King {
    pub color: Color,
    pub has_moved: bool,
}

impl King {
    pub fn new(color: Color) -> Self {
        King {
            color,
            has_moved: false,
        }
    }

    pub fn is_valid_move(&self, board: &Board, start: &Move, end: &Move) -> bool {
        let dx = (start.x - end.x).abs();
        let dy = (start.y - end.y).abs();

        if dx > 1 || dy > 1 {
            return false;
        }

        if let Some(piece) = board.get_piece(end.x, end.y) {
            if piece.color == self.color {
                return false;
            }
        }

        true
    }

    pub fn can_castle(&self, board: &Board, start: &Move, end: &Move) -> bool {
        if self.has_moved {
            return false;
        }

        let dx = end.x - start.x;

        if dx.abs() != 2 {
            return false;
        }

        let rook_x = if dx > 0 { 7 } else { 0 };
        let rook_y = start.y;

        if let Some(piece) = board.get_piece(rook_x, rook_y) {
            if let Piece::Rook(rook) = piece {
                if rook.has_moved {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        let (x_step, y_step) = if dx > 0 { (1, 0) } else { (-1, 0) };
        let mut x = start.x + x_step;
        let mut y = start.y + y_step;

        while x != end.x {
            if board.get_piece(x, y).is_some() {
                return false;
            }

            x += x_step;
            y += y_step;
        }

        true
    }
}
```