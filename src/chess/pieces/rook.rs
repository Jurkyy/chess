```rust
use crate::chess::board::Board;
use crate::chess::move::Move;
use crate::chess::pieces::Piece;

pub struct Rook {
    position: (usize, usize),
    color: Piece,
    has_moved: bool,
}

impl Rook {
    pub fn new(position: (usize, usize), color: Piece) -> Self {
        Rook {
            position,
            color,
            has_moved: false,
        }
    }

    pub fn is_valid_move(&self, board: &Board, new_move: &Move) -> bool {
        // Check if the move is valid for a rook
        // Add your implementation here
    }

    pub fn make_move(&mut self, board: &mut Board, new_move: &Move) {
        // Make the move and update the rook's position
        // Add your implementation here
        self.has_moved = true;
    }

    pub fn is_castling_move(&self, new_move: &Move) -> bool {
        // Check if the move is a castling move
        // Add your implementation here
    }

    pub fn perform_castling(&mut self, board: &mut Board, new_move: &Move) {
        // Perform the castling move and update the rook's position
        // Add your implementation here
        self.has_moved = true;
    }
}
```