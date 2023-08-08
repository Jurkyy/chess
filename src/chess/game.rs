```rust
use crate::chess::board::Board;
use crate::chess::pieces::{Piece, King, Rook};
use crate::chess::move::Move;

pub struct Game {
    board: Board,
    current_turn: Piece,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_turn: Piece::White,
        }
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), &'static str> {
        if self.board.is_valid_move(&mv) {
            if self.board.is_castling_move(&mv) {
                self.board.perform_castling(&mv)?;
            } else {
                self.board.make_move(&mv)?;
            }
            self.switch_turn();
            Ok(())
        } else {
            Err("Invalid move")
        }
    }

    fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Piece::White => Piece::Black,
            Piece::Black => Piece::White,
        };
    }

    pub fn is_in_check(&self) -> bool {
        self.board.is_in_check(self.current_turn)
    }

    pub fn is_in_checkmate(&self) -> bool {
        self.board.is_in_checkmate(self.current_turn)
    }
}
```