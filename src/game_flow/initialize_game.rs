use std::rc::Rc;

use crate::game_components::bishop::Bishop;
use crate::game_components::chessboard::{Chessboard, Color};
use crate::game_components::king::King;
use crate::game_components::knight::Knight;
use crate::game_components::pawn::Pawn;
use crate::game_components::queen::Queen;
use crate::game_components::rook::Rook;
use crate::player_management::player::Player;

pub fn initialize_game() -> (Chessboard, Player, Player) {
    let mut board = Chessboard::new();
    let player1 = Player::new(String::from("Player 1"), Color::White);
    let player2 = Player::new(String::from("Player 2"), Color::Black);

    // Place Pawns on the starting positions
    for i in 0..8 {
        board.place_piece(Rc::new(Pawn::new(Color::White, false)), 1, i);
        board.place_piece(Rc::new(Pawn::new(Color::Black, false)), 6, i);
    }

    // Place the other pieces on the board using the place_piece method
    board.place_piece(Rc::new(Rook::new(Color::White, false)), 0, 0);
    board.place_piece(Rc::new(Rook::new(Color::White, false)), 0, 7);
    board.place_piece(Rc::new(Rook::new(Color::Black, false)), 7, 0);
    board.place_piece(Rc::new(Rook::new(Color::Black, false)), 7, 7);

    board.place_piece(Rc::new(Knight::new(Color::White)), 0, 1);
    board.place_piece(Rc::new(Knight::new(Color::White)), 0, 6);
    board.place_piece(Rc::new(Knight::new(Color::Black)), 7, 1);
    board.place_piece(Rc::new(Knight::new(Color::Black)), 7, 6);

    board.place_piece(Rc::new(Bishop::new(Color::White)), 0, 2);
    board.place_piece(Rc::new(Bishop::new(Color::White)), 0, 5);
    board.place_piece(Rc::new(Bishop::new(Color::Black)), 7, 2);
    board.place_piece(Rc::new(Bishop::new(Color::Black)), 7, 5);

    board.place_piece(Rc::new(Queen::new(Color::White)), 0, 3);
    board.place_piece(Rc::new(Queen::new(Color::Black)), 7, 3);

    board.place_piece(Rc::new(King::new(Color::White, false)), 0, 4);
    board.place_piece(Rc::new(King::new(Color::Black, false)), 7, 4);

    (board, player1, player2)
}
