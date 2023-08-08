```rust
use crate::chess::board::Board;
use crate::chess::pieces::{Piece, King, Rook};
use crate::chess::utils::Position;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
        }
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        let piece = self.board.get_piece(from)?;

        if piece.is_valid_move(from, to) {
            self.board.move_piece(from, to)?;
            Ok(())
        } else {
            Err("Invalid move")
        }
    }

    pub fn castle(&mut self, king: &King, rook: &Rook) -> Result<(), &'static str> {
        if king.can_castle(rook) {
            let king_position = self.board.get_position(king)?;
            let rook_position = self.board.get_position(rook)?;

            self.board.move_piece(king_position, rook_position)?;
            Ok(())
        } else {
            Err("Castling not possible")
        }
    }
}
```