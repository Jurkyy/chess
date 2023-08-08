```rust
use crate::chess::color::Color;

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub struct Piece {
    piece_type: PieceType,
    color: Color,
    has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            has_moved: false,
        }
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}
```