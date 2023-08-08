```rust
use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::castling::Castling;
use crate::chess::pawn::Pawn;
use crate::chess::knight::Knight;
use crate::chess::bishop::Bishop;
use crate::chess::rook::Rook;
use crate::chess::queen::Queen;
use crate::chess::king::King;

pub struct Game {
    board: Board,
    current_turn: PieceColor,
    status: GameStatus,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_turn: PieceColor::White,
            status: GameStatus::Ongoing,
        }
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        let piece = self.board.get_piece(from)?;
        if piece.color != self.current_turn {
            return Err("It's not your turn");
        }
        piece.move_piece(to)?;
        if self.is_in_check(self.current_turn) {
            return Err("You can't put yourself in check");
        }
        self.current_turn = self.current_turn.opposite();
        self.update_status();
        Ok(())
    }

    pub fn is_in_check(&self, color: PieceColor) -> bool {
        let king_position = self.board.get_king_position(color);
        self.board.is_under_attack(king_position, color.opposite())
    }

    pub fn is_in_checkmate(&self, color: PieceColor) -> bool {
        if !self.is_in_check(color) {
            return false;
        }
        self.board.get_all_pieces(color).all(|piece| piece.get_valid_moves().is_empty())
    }

    fn update_status(&mut self) {
        if self.is_in_checkmate(self.current_turn.opposite()) {
            self.status = GameStatus::Checkmate;
        } else if self.is_in_check(self.current_turn) {
            self.status = GameStatus::Check;
        } else if self.board.get_all_pieces(self.current_turn).all(|piece| piece.get_valid_moves().is_empty()) {
            self.status = GameStatus::Stalemate;
        }
    }
}

enum GameStatus {
    Ongoing,
    Check,
    Checkmate,
    Stalemate,
}
```