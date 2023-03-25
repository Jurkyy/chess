mod chessboard;

use chessboard::board::ChessBoard;
fn main() {
    let mut chessboard = ChessBoard::new();

    chessboard.fen();
    chessboard.print_chessboard();

    chessboard.move_piece_from_input("b2 b4");
    chessboard.fen();
    chessboard.print_chessboard();

    chessboard.move_piece_from_input("e7 e5");
    chessboard.fen();
    chessboard.print_chessboard();

    chessboard.move_piece_from_input("b4 b5");
    chessboard.fen();
    chessboard.print_chessboard();

    chessboard.move_piece_from_input("a7 a5");
    chessboard.fen();
    chessboard.print_chessboard();

    chessboard.move_piece_from_input("b5 a6");
    chessboard.fen();
    chessboard.print_chessboard();
}
