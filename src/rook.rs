```rust
use crate::piece::Piece;
use crate::board::Board;
use crate::castling::can_castle;

pub struct Rook {
    piece: Piece,
    has_moved: bool,
}

impl Rook {
    pub fn new(piece: Piece) -> Self {
        Rook {
            piece,
            has_moved: false,
        }
    }

    pub fn move_piece(&mut self, board: &mut Board, to: (usize, usize)) -> Result<(), &'static str> {
        let from = self.piece.position;
        if from.0 == to.0 || from.1 == to.1 {
            if board.is_path_clear(from, to) {
                self.piece.position = to;
                self.has_moved = true;
                Ok(())
            } else {
                Err("Path is not clear for rook to move")
            }
        } else {
            Err("Rook can only move horizontally or vertically")
        }
    }

    pub fn can_castle(&self, board: &Board, king: &Piece) -> bool {
        !self.has_moved && can_castle(board, &self.piece, king)
    }
}
```