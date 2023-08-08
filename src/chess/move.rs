```rust
use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::game::Game;
use crate::chess::castling::Castling;

pub struct Move {
    from: Position,
    to: Position,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    pub fn is_valid_move(&self, game: &Game) -> bool {
        let piece = game.board.get_piece(self.from);
        match piece {
            Some(piece) => piece.is_valid_move(self.from, self.to, &game.board),
            None => false,
        }
    }

    pub fn move_piece(&mut self, game: &mut Game) {
        if self.is_valid_move(game) {
            let piece = game.board.remove_piece(self.from);
            if let Some(piece) = piece {
                game.board.set_piece(self.to, piece);
            }
        }
    }

    pub fn can_castle(&self, game: &Game) -> bool {
        let piece = game.board.get_piece(self.from);
        match piece {
            Some(Piece::King(color)) => {
                let castling = Castling::new(self.from, self.to, color);
                castling.is_valid_castling(&game.board)
            }
            _ => false,
        }
    }
}
```