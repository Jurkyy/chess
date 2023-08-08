```rust
mod game_components;
mod game_logic;
mod player_management;
mod user_interface;
mod game_flow;

use game_components::chessboard::Chessboard;
use player_management::player::Player;
use game_flow::initialize_game::initialize_game;
use game_flow::main_game_loop::main_game_loop;

fn main() {
    let (mut chessboard, mut players) = initialize_game();
    main_game_loop(&mut chessboard, &mut players);
}
```