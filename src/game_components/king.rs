```rust
use crate::game_components::chess_piece::{ChessPiece, Color};

pub struct King {
    color: Color,
}

impl ChessPiece for King {
    fn color(&self) -> &Color {
        &self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'K',
            Color::Black => 'k',
        }
    }

    // Implement other common methods for King
}
```