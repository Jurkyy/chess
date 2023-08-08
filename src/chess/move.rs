```rust
use crate::chess::board::Board;
use crate::chess::pieces::{King, Rook};

pub struct Move {
    from: (usize, usize),
    to: (usize, usize),
    piece: Piece,
    is_castling_move: bool,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize), piece: Piece) -> Self {
        let is_castling_move = match piece {
            Piece::King(king) => king.is_castling_move(from, to),
            Piece::Rook(rook) => rook.is_castling_move(from, to),
            _ => false,
        };

        Self {
            from,
            to,
            piece,
            is_castling_move,
        }
    }

    pub fn execute(&self, board: &mut Board) -> Result<(), &'static str> {
        if !self.is_valid_move(board) {
            return Err("Invalid move");
        }

        if self.is_castling_move {
            match self.piece {
                Piece::King(king) => king.perform_castling(self.from, self.to, board),
                Piece::Rook(rook) => rook.perform_castling(self.from, self.to, board),
                _ => (),
            };
        } else {
            board.make_move(self.from, self.to)?;
        }

        Ok(())
    }

    fn is_valid_move(&self, board: &Board) -> bool {
        match self.piece {
            Piece::King(king) => king.is_valid_move(self.from, self.to, board),
            Piece::Rook(rook) => rook.is_valid_move(self.from, self.to, board),
            _ => board.is_valid_move(self.from, self.to),
        }
    }
}
```