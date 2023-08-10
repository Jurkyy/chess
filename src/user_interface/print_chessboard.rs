use crate::game_components::chessboard::{Chessboard, Square};

pub fn print_chessboard(chessboard: &Chessboard) {
    println!("  a b c d e f g h");
    for row in (0..8).rev() {
        print!("{} ", row + 1);
        for col in 0..8 {
            match &chessboard.get_square(col, row) {
                Square::Empty => print!(". "),
                Square::Occupied(piece) => print!("{} ", piece.ascii_char()),
            }
        }
        println!();
    }
}
