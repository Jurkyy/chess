```rust
mod chess;

use chess::game::Game;
use chess::board::Board;
use chess::piece::Piece;
use chess::move::Move;
use chess::castling::Castling;

fn main() {
    let mut game = Game::new();
    game.start();
}
```