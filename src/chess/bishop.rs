```rust
use super::piece::Piece;
use super::color::Color;
use super::board::Board;

pub struct Bishop {
    color: Color,
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Bishop { color }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn is_valid_move(&self, board: &Board, start: (usize, usize), end: (usize, usize)) -> bool {
        let dx = (end.0 as i32 - start.0 as i32).abs();
        let dy = (end.1 as i32 - start.1 as i32).abs();

        if dx == dy {
            let direction = (
                (end.0 as i32 - start.0 as i32) / dx,
                (end.1 as i32 - start.1 as i32) / dy,
            );

            let mut current = start;
            current.0 = (current.0 as i32 + direction.0) as usize;
            current.1 = (current.1 as i32 + direction.1) as usize;

            while current != end {
                if board.get_piece(current).is_some() {
                    return false;
                }

                current.0 = (current.0 as i32 + direction.0) as usize;
                current.1 = (current.1 as i32 + direction.1) as usize;
            }

            return board.get_piece(end).map_or(true, |p| p.color() != self.color());
        }

        false
    }
}

impl Piece for Bishop {
    fn color(&self) -> &Color {
        &self.color
    }

    fn can_move(&self, board: &Board, start: (usize, usize), end: (usize, usize)) -> bool {
        self.is_valid_move(board, start, end)
    }
}
```