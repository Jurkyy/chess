mod chessboard;

use chessboard::board::ChessBoard;
fn main() {
    let chessboard = ChessBoard::new();

    chessboard.fen();
    chessboard.print_chessboard();
}
