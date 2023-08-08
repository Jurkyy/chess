```rust
use super::piece::Piece;
use super::color::Color;
use super::board::Board;
use super::move::Move;

pub struct Pawn {
    color: Color,
    first_move: bool,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Pawn {
            color,
            first_move: true,
        }
    }

    pub fn is_valid_move(&self, start: (usize, usize), end: (usize, usize), board: &Board) -> bool {
        let dx = (end.0 as i32 - start.0 as i32).abs();
        let dy = end.1 as i32 - start.1 as i32;

        match self.color {
            Color::White => {
                if dy == -1 && dx == 0 && board.get_piece(end).is_none() {
                    true
                } else if dy == -1 && dx == 1 && board.get_piece(end).is_some() {
                    true
                } else if dy == -2 && dx == 0 && self.first_move && board.get_piece(end).is_none() {
                    true
                } else {
                    false
                }
            }
            Color::Black => {
                if dy == 1 && dx == 0 && board.get_piece(end).is_none() {
                    true
                } else if dy == 1 && dx == 1 && board.get_piece(end).is_some() {
                    true
                } else if dy == 2 && dx == 0 && self.first_move && board.get_piece(end).is_none() {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn en_passant(&self, start: (usize, usize), end: (usize, usize), board: &mut Board) -> bool {
        let dx = (end.0 as i32 - start.0 as i32).abs();
        let dy = end.1 as i32 - start.1 as i32;

        match self.color {
            Color::White => {
                if dy == -1 && dx == 1 && board.get_piece((end.0, end.1 + 1)).is_some() {
                    board.remove_piece((end.0, end.1 + 1));
                    true
                } else {
                    false
                }
            }
            Color::Black => {
                if dy == 1 && dx == 1 && board.get_piece((end.0, end.1 - 1)).is_some() {
                    board.remove_piece((end.0, end.1 - 1));
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn two_tile_move(&mut self, start: (usize, usize), end: (usize, usize), board: &mut Board) -> bool {
        if self.is_valid_move(start, end, board) {
            self.first_move = false;
            true
        } else {
            false
        }
    }
}

impl Piece for Pawn {
    fn color(&self) -> &Color {
        &self.color
    }

    fn move_piece(&mut self, start: (usize, usize), end: (usize, usize), board: &mut Board) -> bool {
        if self.two_tile_move(start, end, board) || self.en_passant(start, end, board) {
            true
        } else {
            false
        }
    }
}
```