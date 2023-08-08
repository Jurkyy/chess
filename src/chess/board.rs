```rust
use crate::chess::pieces::{Piece, King, Rook};
use crate::chess::move::Move;

pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
    pub white_king_moved: bool,
    pub black_king_moved: bool,
    pub white_rook_moved: [bool; 2],
    pub black_rook_moved: [bool; 2],
}

impl Board {
    pub fn new() -> Self {
        // Initialize the board with pieces at their initial positions
        // and set the *_moved flags to false
    }

    pub fn is_valid_move(&self, mv: &Move) -> bool {
        // Check if the move is valid
    }

    pub fn make_move(&mut self, mv: Move) {
        // Make the move and update the *_moved flags if a king or rook is moved
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        // Check if the king of the given color is in check
    }

    pub fn is_in_checkmate(&self, color: Color) -> bool {
        // Check if the king of the given color is in checkmate
    }

    pub fn is_castling_move(&self, mv: &Move) -> bool {
        // Check if the move is a castling move
    }

    pub fn perform_castling(&mut self, mv: Move) {
        // Perform the castling move
    }
}
```