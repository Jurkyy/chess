```rust
pub trait Piece {
    fn new(position: (usize, usize), is_white: bool) -> Self;
    fn position(&self) -> (usize, usize);
    fn color(&self) -> bool;
    fn move_piece(&mut self, to: (usize, usize));
    fn valid_move(&self, to: (usize, usize)) -> bool;
}
```