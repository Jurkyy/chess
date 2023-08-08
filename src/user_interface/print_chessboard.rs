```rust
use crate::game_components::chessboard::Chessboard;
use crate::game_components::chess_piece::ChessPiece;

pub fn print_chessboard(chessboard: &Chessboard) {
    for row in 0..8 {
        for col in 0..8 {
            match &chessboard.squares[row][col] {
                Square::Empty => print!(". "),
                Square::Occupied(piece) => print!("{} ", piece.ascii_char()),
            }
        }
        println!();
    }
}
```