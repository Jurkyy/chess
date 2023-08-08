```rust
use super::piece::Piece;
use super::color::Color;
use super::board::Board;

pub struct Knight {
    pub color: Color,
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Knight { color }
    }

    pub fn is_valid_move(&self, board: &Board, start: (usize, usize), end: (usize, usize)) -> bool {
        let dx = (start.0 as i32 - end.0 as i32).abs();
        let dy = (start.1 as i32 - end.1 as i32).abs();

        if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
            if let Some(piece) = board.get_piece(end) {
                return piece.color != self.color;
            }
            return true;
        }
        false
    }
}

impl Piece for Knight {
    fn color(&self) -> &Color {
        &self.color
    }
}
```