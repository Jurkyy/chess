```rust
mod chess;

use chess::game::Game;
use chess::board::Board;
use chess::pawn::Pawn;
use chess::knight::Knight;
use chess::bishop::Bishop;
use chess::rook::Rook;
use chess::queen::Queen;
use chess::king::King;
use chess::castling::Castling;

fn main() {
    let mut game = Game::new();
    game.start();
}
```