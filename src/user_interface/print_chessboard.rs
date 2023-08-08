use crate::game_components::chessboard::{Chessboard, Square};

pub fn print_chessboard(chessboard: &Chessboard) {
    for row in 0..8 {
        for col in 0..8 {
            match &chessboard.get_square(row, col) {
                Square::Empty => print!(". "),
                Square::Occupied(piece) => print!("{} ", piece.ascii_char()),
            }
        }
        println!();
    }
}
