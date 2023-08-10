use crate::game_components::{
    chess_piece::ChessPiece,
    chessboard::{Chessboard, Square},
};

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

pub fn print_valid_moves(piece: &dyn ChessPiece, start: (usize, usize), chessboard: &Chessboard) {
    let valid_moves = piece.valid_moves(start, chessboard);

    println!("Valid moves {:?} for {}: ", valid_moves, piece.ascii_char());

    for row in (0..8).rev() {
        print!("{} ", row + 1);
        for col in 0..8 {
            let position = (col, row);
            if valid_moves.contains(&position) {
                print!("* ");
            } else {
                match &chessboard.get_square(col, row) {
                    Square::Empty => print!(". "),
                    Square::Occupied(p) => print!("{} ", p.ascii_char()),
                }
            }
        }
        println!();
    }

    println!("  a b c d e f g h");
}
