use std::rc::Rc;

use crate::game_components::chess_piece::ChessPiece;

use super::{
    chessboard::{Chessboard, Color, Square},
    rook::Rook,
};

pub struct King {
    color: Color,
    has_moved: bool,
}

impl King {
    pub fn new(color: Color, has_moved: bool) -> Self {
        King { color, has_moved }
    }
    fn can_castle(&self, chessboard: &Chessboard, king_pos: (usize, usize)) -> bool {
        // Check if the king has moved
        if self.has_moved {
            return false;
        }

        // Check if the rooks are in their initial positions
        let rook_positions = if self.color == Color::White {
            [(0, 0), (7, 0)]
        } else {
            [(0, 7), (7, 7)]
        };

        for &(x, y) in &rook_positions {
            if let Square::Occupied(piece) = chessboard.get_square(x, y) {
                if let Some(rook) = piece.as_any().downcast_ref::<Rc<Rook>>() {
                    if !rook.has_moved() {
                        // Check if the squares between king and rook are empty
                        let (start, end) = if x < king_pos.0 {
                            (x, king_pos.0)
                        } else {
                            (king_pos.0, x)
                        };

                        for col in start + 1..end {
                            match *chessboard.get_square(col, y) {
                                Square::Empty => {}
                                Square::Occupied(_) => return false,
                            }
                        }

                        // Check if the king is not in check during the move
                        let mut temp_board = chessboard.clone();
                        temp_board.update((king_pos.0, king_pos.1, king_pos.0 + 2, king_pos.1));
                        if temp_board.is_in_check(self.color) {
                            return false;
                        }

                        return true;
                    }
                }
            }
        }

        false
    }
}

impl ChessPiece for King {
    fn color(&self) -> Color {
        self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'K',
            Color::Black => 'k',
        }
    }

    fn valid_moves(&self, start: (usize, usize), chessboard: &Chessboard) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let directions = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ];

        let king_pos = start; // Cache the king's position

        for dir in &directions {
            let x = king_pos.0 as i32 + dir.0;
            let y = king_pos.1 as i32 + dir.1;

            if x >= 0 && y >= 0 && x < 8 && y < 8 {
                match chessboard.get_square(x as usize, y as usize) {
                    Square::Empty => {
                        // Check if moving to this square would not put the king in check
                        let mut temp_board = chessboard.clone();
                        temp_board.update((king_pos.0, king_pos.1, x as usize, y as usize));
                        if !temp_board.is_in_check(self.color) {
                            moves.push((x as usize, y as usize));
                        }
                    }
                    Square::Occupied(piece) => {
                        if piece.color() != self.color() {
                            // Check if moving to this square would not put the king in check
                            let mut temp_board = chessboard.clone();
                            temp_board.update((king_pos.0, king_pos.1, x as usize, y as usize));
                            if !temp_board.is_in_check(self.color) {
                                moves.push((x as usize, y as usize));
                            }
                        }
                    }
                }
            }
        }

        // Add castling moves if applicable
        if self.can_castle(chessboard, king_pos) {
            moves.push((king_pos.0 + 2, king_pos.1));
            moves.push((king_pos.0 - 2, king_pos.1));
        }

        moves
    }
}
