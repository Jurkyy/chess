```rust
pub mod king;
pub mod rook;
pub mod pawn;
pub mod knight;
pub mod bishop;
pub mod queen;

use crate::chess::board::Board;
use crate::chess::utils::Position;

pub trait Piece {
    fn new(position: Position, is_white: bool) -> Self;
    fn position(&self) -> Position;
    fn is_white(&self) -> bool;
    fn is_valid_move(&self, board: &Board, new_position: Position) -> bool;
}
```