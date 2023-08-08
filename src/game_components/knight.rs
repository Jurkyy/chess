```rust
use crate::game_components::chess_piece::{ChessPiece, Color};

pub struct Knight {
    color: Color,
}

impl ChessPiece for Knight {
    fn color(&self) -> &Color {
        &self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'N',
            Color::Black => 'n',
        }
    }
}
```