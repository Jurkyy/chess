```rust
use super::board::Board;
use super::color::Color;
use super::king::King;
use super::rook::Rook;
use super::piece::Piece;

pub struct Castling {
    king: King,
    rook: Rook,
    board: Board,
}

impl Castling {
    pub fn new(king: King, rook: Rook, board: Board) -> Castling {
        Castling { king, rook, board }
    }

    pub fn is_valid_castling(&self) -> bool {
        if self.king.color != self.rook.color {
            return false;
        }

        if self.king.has_moved || self.rook.has_moved {
            return false;
        }

        let king_position = self.board.get_piece_position(&Piece::King(self.king.clone()));
        let rook_position = self.board.get_piece_position(&Piece::Rook(self.rook.clone()));

        if king_position.is_none() || rook_position.is_none() {
            return false;
        }

        let king_position = king_position.unwrap();
        let rook_position = rook_position.unwrap();

        if king_position.0 != rook_position.0 {
            return false;
        }

        let min_position = std::cmp::min(king_position.1, rook_position.1);
        let max_position = std::cmp::max(king_position.1, rook_position.1);

        for position in (min_position + 1)..max_position {
            if self.board.get_piece((king_position.0, position)).is_some() {
                return false;
            }
        }

        true
    }

    pub fn perform_castling(&mut self) {
        if !self.is_valid_castling() {
            return;
        }

        let king_position = self.board.get_piece_position(&Piece::King(self.king.clone())).unwrap();
        let rook_position = self.board.get_piece_position(&Piece::Rook(self.rook.clone())).unwrap();

        if king_position.1 < rook_position.1 {
            self.board.move_piece(king_position, (king_position.0, king_position.1 + 2));
            self.board.move_piece(rook_position, (rook_position.0, rook_position.1 - 2));
        } else {
            self.board.move_piece(king_position, (king_position.0, king_position.1 - 2));
            self.board.move_piece(rook_position, (rook_position.0, rook_position.1 + 3));
        }

        self.king.has_moved = true;
        self.rook.has_moved = true;
    }
}
```