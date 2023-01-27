mod chessboard;

use chessboard::board::ChessBoard;

fn main() {
    let mut chessboard = ChessBoard::new();

    chessboard.fen();
    chessboard.print_chessboard();

    chessboard.move_piece_from_input("e2 e4");
    chessboard.move_piece_from_input("e1 e2");
    chessboard.move_piece_from_input("e2 e3");
    chessboard.move_piece_from_input("e3 f4");
    chessboard.move_piece_from_input("f4 e5");
    chessboard.move_piece_from_input("e5 e6");
    chessboard.move_piece_from_input("e6 e7");

    chessboard.fen();
    chessboard.print_chessboard();
}
