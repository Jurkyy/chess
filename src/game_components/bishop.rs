```rust
use super::chess_piece::ChessPiece;
use super::chess_piece::Color;

pub struct Bishop {
    color: Color,
}

impl ChessPiece for Bishop {
    fn color(&self) -> &Color {
        &self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'B',
            Color::Black => 'b',
        }
    }
}
```