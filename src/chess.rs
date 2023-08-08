```rust
mod piece;
mod pawn;
mod knight;
mod bishop;
mod rook;
mod queen;
mod king;
mod board;
mod castling;

use piece::Piece;
use pawn::Pawn;
use knight::Knight;
use bishop::Bishop;
use rook::Rook;
use queen::Queen;
use king::King;
use board::Board;
use castling::Castling;

pub struct Chess {
    board: Board,
    pieces: Vec<Piece>,
    castling: Castling,
}

impl Chess {
    pub fn new() -> Chess {
        let mut pieces = Vec::new();
        pieces.extend(Pawn::new_set(Color::White));
        pieces.extend(Knight::new_set(Color::White));
        pieces.extend(Bishop::new_set(Color::White));
        pieces.extend(Rook::new_set(Color::White));
        pieces.extend(Queen::new_set(Color::White));
        pieces.extend(King::new_set(Color::White));

        pieces.extend(Pawn::new_set(Color::Black));
        pieces.extend(Knight::new_set(Color::Black));
        pieces.extend(Bishop::new_set(Color::Black));
        pieces.extend(Rook::new_set(Color::Black));
        pieces.extend(Queen::new_set(Color::Black));
        pieces.extend(King::new_set(Color::Black));

        Chess {
            board: Board::new(pieces.clone()),
            pieces,
            castling: Castling::new(),
        }
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        let piece = self.board.get_piece(from)?;
        piece.move_to(to, &self.board)?;
        self.board.update_piece_position(piece, to);
        self.castling.update_castling_rights(piece);
        Ok(())
    }

    pub fn get_piece(&self, position: Position) -> Option<&Piece> {
        self.board.get_piece(position).ok()
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        self.board.is_checkmate(color)
    }
}
```