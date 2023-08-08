```rust
use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::move::Move;
use crate::chess::castling::Castling;

pub struct Game {
    board: Board,
    current_turn: Piece,
    castling: Castling,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_turn: Piece::White,
            castling: Castling::new(),
        }
    }

    pub fn move_piece(&mut self, start: Position, end: Position) -> Result<(), &'static str> {
        let move = Move::new(start, end);
        if self.is_valid_move(&move) {
            self.board.move_piece(move);
            self.switch_turn();
            Ok(())
        } else {
            Err("Invalid move")
        }
    }

    pub fn is_valid_move(&self, move: &Move) -> bool {
        self.board.is_valid_move(move) && self.current_turn == move.piece.color
    }

    pub fn can_castle(&self, castling: &Castling) -> bool {
        self.castling.can_castle(castling)
    }

    fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Piece::White => Piece::Black,
            Piece::Black => Piece::White,
        };
    }
}
```