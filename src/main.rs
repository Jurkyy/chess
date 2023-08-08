mod game_components;
mod game_flow;
mod game_logic;
mod player_management;
mod user_interface;

use game_flow::initialize_game::initialize_game;
use game_flow::main_game_loop::main_game_loop;

fn main() {
    let (mut chessboard, player1, player2) = initialize_game();
    main_game_loop(&mut chessboard, player1, player2);
}
