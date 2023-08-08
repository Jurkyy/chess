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
        board.place_piece(Rc::new(Pawn::new(Color::White, false)), i, 1);
        board.place_piece(Rc::new(Pawn::new(Color::Black, false)), i, 6);
    }

    let white_rook = Rc::new(Rook::new(Color::White, false));
    let black_rook = Rc::new(Rook::new(Color::Black, false));

    let white_knight = Rc::new(Knight::new(Color::White));
    let black_knight = Rc::new(Knight::new(Color::Black));

    let white_bishop = Rc::new(Bishop::new(Color::White));
    let black_bishop = Rc::new(Bishop::new(Color::Black));

    let white_queen = Rc::new(Queen::new(Color::White));
    let black_queen = Rc::new(Queen::new(Color::Black));

    let white_king = Rc::new(King::new(Color::White, false));
    let black_king = Rc::new(King::new(Color::Black, false));

    // Place the other pieces on the board using the place_piece method
    board.place_piece(white_rook.clone(), 0, 0);
    board.place_piece(white_rook.clone(), 7, 0);
    board.place_piece(black_rook.clone(), 0, 7);
    board.place_piece(black_rook.clone(), 7, 7);

    board.place_piece(white_knight.clone(), 1, 0);
    board.place_piece(white_knight.clone(), 6, 0);
    board.place_piece(black_knight.clone(), 1, 7);
    board.place_piece(black_knight.clone(), 6, 7);

    board.place_piece(white_bishop.clone(), 2, 0);
    board.place_piece(white_bishop.clone(), 5, 0);
    board.place_piece(black_bishop.clone(), 2, 7);
    board.place_piece(black_bishop.clone(), 5, 7);

    board.place_piece(white_queen, 3, 0);
    board.place_piece(black_queen, 3, 7);

    board.place_piece(white_king, 4, 0);
    board.place_piece(black_king, 4, 7);

    (board, player1, player2)
}
