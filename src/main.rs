mod board;
mod pieces;

use board::Board;
use pieces::{Pawn, Knight, Bishop, Rook, Queen, King};

fn main() {
    let mut board = Board::new();

    // Initialize the board with pieces
    board.init();

    // Game loop
    loop {
        // Display the board
        board.display();

        // Get the player's move
        let move = board.get_move();

        // Make the move
        board.make_move(move);

        // Check for game end conditions
        if board.checkmate() {
            println!("Checkmate!");
            break;
        } else if board.stalemate() {
            println!("Stalemate!");
            break;
        }
    }
}