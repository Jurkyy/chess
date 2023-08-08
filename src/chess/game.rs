```rust
use crate::chess::board::Board;
use crate::chess::move::Move;
use crate::chess::piece::Piece;
use crate::chess::color::Color;
use crate::chess::castling::Castling;

pub struct Game {
    board: Board,
    current_color: Color,
    castling: Castling,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_color: Color::White,
            castling: Castling::new(),
        }
    }

    pub fn make_move(&mut self, start: (usize, usize), end: (usize, usize)) -> Result<(), &'static str> {
        let piece = self.board.get_piece(start)?;

        if piece.color != self.current_color {
            return Err("It's not your turn");
        }

        let move = Move::new(start, end);

        if !piece.is_valid_move(&move, &self.board) {
            return Err("Invalid move");
        }

        if let Some(castling_move) = self.castling.check_castling(&move, &piece, &self.board) {
            self.board.move_piece(castling_move)?;
        } else {
            self.board.move_piece(move)?;
        }

        self.current_color = match self.current_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        Ok(())
    }
}
```