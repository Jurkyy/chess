```rust
pub enum Color {
    White,
    Black,
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    piece_type: PieceType,
    color: Color,
    position: (usize, usize),
    has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color, position: (usize, usize)) -> Self {
        Self {
            piece_type,
            color,
            position,
            has_moved: false,
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    pub fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}
```