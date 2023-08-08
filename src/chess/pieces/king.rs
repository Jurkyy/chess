```rust
use crate::chess::board::Board;
use crate::chess::move::Move;
use crate::chess::pieces::rook::Rook;

pub struct King {
    position: (usize, usize),
    has_moved: bool,
}

impl King {
    pub fn new(position: (usize, usize)) -> Self {
        King {
            position,
            has_moved: false,
        }
    }

    pub fn is_valid_move(&self, board: &Board, m: &Move) -> bool {
        // Check if the move is a valid king move or a valid castling move
        self.is_valid_king_move(m) || self.is_castling_move(board, m)
    }

    pub fn make_move(&mut self, m: &Move) {
        self.position = m.to;
        self.has_moved = true;
    }

    fn is_valid_king_move(&self, m: &Move) -> bool {
        // Check if the move is a valid king move
        let dx = (self.position.0 as i32 - m.to.0 as i32).abs();
        let dy = (self.position.1 as i32 - m.to.1 as i32).abs();
        dx <= 1 && dy <= 1
    }

    fn is_castling_move(&self, board: &Board, m: &Move) -> bool {
        // Check if the move is a valid castling move
        if self.has_moved {
            return false;
        }

        let dx = self.position.0 as i32 - m.to.0 as i32;
        if dx.abs() != 2 {
            return false;
        }

        let rook_position = if dx > 0 { (0, self.position.1) } else { (7, self.position.1) };
        let rook = board.get_piece(rook_position);
        if let Some(Piece::Rook(rook)) = rook {
            if !rook.has_moved() && rook.is_valid_castling_path(board, self.position) {
                return true;
            }
        }

        false
    }

    pub fn perform_castling(&self, board: &mut Board, m: &Move) {
        // Perform a castling move
        let dx = self.position.0 as i32 - m.to.0 as i32;
        let rook_position = if dx > 0 { (0, self.position.1) } else { (7, self.position.1) };
        let new_rook_position = if dx > 0 { (3, self.position.1) } else { (5, self.position.1) };

        board.move_piece(rook_position, new_rook_position);
        board.move_piece(self.position, m.to);
    }
}
```