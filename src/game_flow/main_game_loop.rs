use crate::game_components::chessboard::Chessboard;
use crate::game_flow::winning_condition::{check_winning_condition, Ending};
use crate::game_logic::move_validation::validate_move;
use crate::player_management::player::Player;
use crate::user_interface::input_handling::handle_input;
use crate::user_interface::print_chessboard::print_chessboard;

pub fn main_game_loop(chessboard: &mut Chessboard, player1: Player, player2: Player) {
    let mut gameloop = GameLoop::new(player1, player2);

    loop {
        print_chessboard(chessboard);

        let current_player = gameloop.get_player();
        println!("{}'s turn", current_player.name());

        match handle_input() {
            Ok((x1, y1, x2, y2)) => {
                println!("Move coordinates: ({}, {}) to ({}, {})", x1, y1, x2, y2);
                if validate_move(&chessboard, (x1, y1), (x2, y2)) {
                    match chessboard.update((x1, y1, x2, y2)) {
                        Ok(_) => continue,
                        Err(err) => eprintln!("{}", err),
                    }
                } else {
                    println!("Invalid move, try again");
                    continue;
                }

                match check_winning_condition(current_player, chessboard) {
                    Some((ending, player)) => match ending {
                        Ending::Victory => {
                            println!("{} has won the game!", player.name());
                            break;
                        }
                        Ending::Tie => {
                            println!(
                                "The game is a tie, but player {} won... Something went wrong",
                                player.name()
                            );
                            break;
                        }
                    },
                    None => {
                        gameloop.switch_player();
                    }
                }
            }
            Err(error_message) => {
                eprintln!("Error: {}", error_message);
                // Handle the error case
            }
        }
    }
}

struct GameLoop {
    current_player: u8,
    player1: Player,
    player2: Player,
}

impl GameLoop {
    fn new(player1: Player, player2: Player) -> Self {
        GameLoop {
            current_player: 0,
            player1,
            player2,
        }
    }

    fn get_player(&self) -> &Player {
        if self.current_player == 0 {
            &self.player1
        } else {
            &self.player2
        }
    }

    fn switch_player(&mut self) {
        if self.current_player == 0 {
            self.current_player = 1;
        } else {
            self.current_player = 0;
        }
    }
}
