```rust
pub enum Color {
    White,
    Black,
}

pub trait ChessPiece {
    fn color(&self) -> Color;
    fn ascii_char(&self) -> char;
    // Other common methods for pieces can be added here
}
```