```rust
use crate::game_components::chess_piece::{ChessPiece, Color};

pub struct Rook {
    color: Color,
}

impl ChessPiece for Rook {
    fn color(&self) -> &Color {
        &self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'R',
            Color::Black => 'r',
        }
    }
}
```