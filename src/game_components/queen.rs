```rust
use super::chess_piece::ChessPiece;
use super::chess_piece::Color;

pub struct Queen {
    color: Color,
}

impl ChessPiece for Queen {
    fn color(&self) -> &Color {
        &self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'Q',
            Color::Black => 'q',
        }
    }
}
```