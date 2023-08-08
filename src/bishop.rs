```rust
use crate::piece::Piece;
use crate::board::Board;

pub struct Bishop {
    piece: Piece,
}

impl Bishop {
    pub fn new(piece: Piece) -> Self {
        Bishop { piece }
    }

    pub fn valid_move(&self, board: &Board, end_x: usize, end_y: usize) -> bool {
        let start_x = self.piece.x;
        let start_y = self.piece.y;

        if start_x == end_x || start_y == end_y {
            return false;
        }

        if (start_x as i32 - end_x as i32).abs() != (start_y as i32 - end_y as i32).abs() {
            return false;
        }

        let x_direction = if start_x < end_x { 1 } else { -1 };
        let y_direction = if start_y < end_y { 1 } else { -1 };
        let mut x = start_x as i32 + x_direction;
        let mut y = start_y as i32 + y_direction;

        while x != end_x as i32 && y != end_y as i32 {
            if board.get_piece(x as usize, y as usize).is_some() {
                return false;
            }
            x += x_direction;
            y += y_direction;
        }

        true
    }
}
```