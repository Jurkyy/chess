```rust
use super::chess_piece::ChessPiece;
use super::chess_piece::Color;

pub struct Pawn {
    color: Color,
}

impl ChessPiece for Pawn {
    fn color(&self) -> &Color {
        &self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'P',
            Color::Black => 'p',
        }
    }
}
```