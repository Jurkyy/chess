```rust
use crate::game_components::chessboard::Chessboard;
use crate::player_management::player::Player;
use crate::user_interface::print_chessboard::print_chessboard;
use crate::user_interface::input_handling::handle_input;
use crate::game_logic::move_validation::validate_move;
use crate::game_flow::winning_condition::check_winning_condition;

pub fn main_game_loop(players: &mut [Player; 2], chessboard: &mut Chessboard) {
    let mut current_player_index = 0;

    loop {
        print_chessboard(chessboard);

        let current_player = &players[current_player_index];
        println!("{}'s turn", current_player.name);

        let move = handle_input(current_player);
        if validate_move(move, current_player, chessboard) {
            chessboard.update(move);
        } else {
            println!("Invalid move, try again");
            continue;
        }

        if check_winning_condition(current_player, chessboard) {
            println!("{} has won the game!", current_player.name);
            break;
        }

        current_player_index = (current_player_index + 1) % players.len();
    }
}
```