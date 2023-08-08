```rust
mod chess;
mod piece;
mod pawn;
mod knight;
mod bishop;
mod rook;
mod queen;
mod king;
mod board;
mod castling;

use chess::Chess;
use board::Board;

fn main() {
    let mut game = Chess::new();
    game.init_board();
    game.play();
}
```