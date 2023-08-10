use crate::game_components::chessboard::{Chessboard, Square};

pub fn capture_piece(chessboard: &mut Chessboard, target_square: (usize, usize)) {
    let square = &mut chessboard.get_square(target_square.0, target_square.1);

    match square {
        Square::Occupied(piece) => {
            println!("{} piece captured!", piece.color());
            *square = &Square::Empty;
        }
        Square::Empty => {
            println!("No piece at the target square to capture!");
        }
    }
}
