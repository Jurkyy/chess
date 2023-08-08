```rust
use super::board::Board;
use super::piece::Piece;
use super::castling::can_castle;

pub struct King {
    pub position: (usize, usize),
    pub color: String,
    pub has_moved: bool,
}

impl King {
    pub fn new(position: (usize, usize), color: String) -> King {
        King {
            position,
            color,
            has_moved: false,
        }
    }

    pub fn is_valid_move(&self, board: &Board, end: (usize, usize)) -> bool {
        let dx = (self.position.0 as i32 - end.0 as i32).abs();
        let dy = (self.position.1 as i32 - end.1 as i32).abs();

        if dx > 1 || dy > 1 {
            return false;
        }

        if let Some(piece) = board.get_piece(end) {
            if piece.color == self.color {
                return false;
            }
        }

        true
    }

    pub fn move_piece(&mut self, board: &mut Board, end: (usize, usize)) -> Result<(), &'static str> {
        if self.is_valid_move(board, end) {
            let piece = board.remove_piece(self.position).unwrap();
            board.place_piece(end, piece);
            self.position = end;
            self.has_moved = true;
            Ok(())
        } else {
            Err("InvalidMoveError")
        }
    }

    pub fn can_castle(&self, board: &Board, rook: &Piece) -> bool {
        if self.has_moved || rook.has_moved {
            return false;
        }

        can_castle(board, self.position, rook.position)
    }

    pub fn perform_castling(&mut self, board: &mut Board, rook: &mut Piece) -> Result<(), &'static str> {
        if self.can_castle(board, rook) {
            let king_end = if self.position.1 < rook.position.1 { (self.position.0, 6) } else { (self.position.0, 2) };
            let rook_end = if self.position.1 < rook.position.1 { (self.position.0, 5) } else { (self.position.0, 3) };

            self.move_piece(board, king_end)?;
            rook.move_piece(board, rook_end)?;

            Ok(())
        } else {
            Err("InvalidMoveError")
        }
    }
}
```