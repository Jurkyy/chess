```rust
use crate::chess::board::Board;
use crate::chess::game::Game;
use crate::chess::move::Move;
use crate::chess::piece::Piece;
use crate::chess::piece::Color;

pub struct Castling {
    king_side: bool,
    queen_side: bool,
}

impl Castling {
    pub fn new() -> Castling {
        Castling {
            king_side: false,
            queen_side: false,
        }
    }

    pub fn can_castle(&self, game: &Game, color: Color) -> bool {
        if color == Color::White {
            self.king_side = game.board.is_empty((7, 6)) && game.board.is_empty((7, 5));
            self.queen_side = game.board.is_empty((7, 2)) && game.board.is_empty((7, 3)) && game.board.is_empty((7, 4));
        } else {
            self.king_side = game.board.is_empty((0, 6)) && game.board.is_empty((0, 5));
            self.queen_side = game.board.is_empty((0, 2)) && game.board.is_empty((0, 3)) && game.board.is_empty((0, 4));
        }

        self.king_side || self.queen_side
    }

    pub fn perform_castling(&mut self, game: &mut Game, mv: Move) {
        if self.can_castle(game, mv.piece.color) {
            if self.king_side {
                game.board.move_piece(mv.from, (mv.from.0, 6));
                game.board.move_piece((mv.from.0, 7), (mv.from.0, 5));
            } else if self.queen_side {
                game.board.move_piece(mv.from, (mv.from.0, 2));
                game.board.move_piece((mv.from.0, 0), (mv.from.0, 3));
            }
        }
    }
}
```