```rust
use crate::chess::piece::Piece;
use crate::chess::board::Board;
use crate::chess::color::Color;
use crate::chess::castling::Castling;
use crate::chess::king::King;
use crate::chess::rook::Rook;
use crate::chess::pawn::Pawn;
use crate::chess::bishop::Bishop;
use crate::chess::knight::Knight;

pub struct Move {
    from: (usize, usize),
    to: (usize, usize),
    piece: Piece,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize), piece: Piece) -> Self {
        Self { from, to, piece }
    }

    pub fn is_valid_move(&self, board: &Board) -> bool {
        match self.piece {
            Piece::King(king) => king.is_valid_move(self.from, self.to, board),
            Piece::Rook(rook) => rook.is_valid_move(self.from, self.to, board),
            Piece::Pawn(pawn) => pawn.is_valid_move(self.from, self.to, board),
            Piece::Bishop(bishop) => bishop.is_valid_move(self.from, self.to, board),
            Piece::Knight(knight) => knight.is_valid_move(self.from, self.to, board),
            _ => false,
        }
    }

    pub fn execute_move(&mut self, board: &mut Board) {
        if self.is_valid_move(board) {
            let piece = board.get_piece(self.from).unwrap();
            board.set_piece(self.to, piece);
            board.remove_piece(self.from);
        }
    }

    pub fn execute_castling(&mut self, board: &mut Board, castling: Castling) {
        if castling.is_valid_castling(board) {
            let king = board.get_piece(castling.king_position).unwrap();
            let rook = board.get_piece(castling.rook_position).unwrap();
            board.set_piece(castling.new_king_position, king);
            board.set_piece(castling.new_rook_position, rook);
            board.remove_piece(castling.king_position);
            board.remove_piece(castling.rook_position);
        }
    }
}
```