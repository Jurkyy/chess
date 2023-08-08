```rust
use super::piece::Piece;
use super::color::Color;
use super::board::Board;
use super::move::Move;

pub struct Rook {
    color: Color,
    has_moved: bool,
}

impl Rook {
    pub fn new(color: Color) -> Self {
        Rook {
            color,
            has_moved: false,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}

impl Piece for Rook {
    fn color(&self) -> &Color {
        &self.color
    }

    fn can_move(&self, board: &Board, start: &Move, end: &Move) -> bool {
        let dx = (end.x - start.x).abs();
        let dy = (end.y - start.y).abs();

        if dx == 0 && dy != 0 {
            // vertical move
            let direction = if start.y < end.y { 1 } else { -1 };
            let mut y = start.y + direction;
            while y != end.y {
                if board.get_piece(start.x, y).is_some() {
                    return false;
                }
                y += direction;
            }
        } else if dy == 0 && dx != 0 {
            // horizontal move
            let direction = if start.x < end.x { 1 } else { -1 };
            let mut x = start.x + direction;
            while x != end.x {
                if board.get_piece(x, start.y).is_some() {
                    return false;
                }
                x += direction;
            }
        } else {
            return false;
        }

        true
    }
}
```