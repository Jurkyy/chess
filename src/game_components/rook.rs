use crate::game_components::chess_piece::ChessPiece;

use super::chessboard::{Chessboard, Color, Square};

pub struct Rook {
    color: Color,
    has_moved: bool,
}

impl Rook {
    pub fn new(color: Color, has_moved: bool) -> Self {
        Rook { color, has_moved }
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn can_castle(&self, chessboard: &Chessboard, rook_pos: (usize, usize)) -> bool {
        // Check if the rook has moved
        if self.has_moved {
            return false;
        }

        // Check if the king has moved
        let king_pos = chessboard.find_king(self.color);
        if king_pos.0 != usize::MAX {
            let king_row = king_pos.0;
            let king_col = king_pos.1;

            if self.color == Color::White {
                if king_row != 7 {
                    return false;
                }

                // Check if the squares between king and rook are empty
                let (start, end) = if rook_pos.1 < king_col {
                    (rook_pos.1, king_col)
                } else {
                    (king_col, rook_pos.1)
                };

                for col in start + 1..end {
                    match *chessboard.get_square(king_row, col) {
                        Square::Empty => {}
                        Square::Occupied(_) => return false,
                    }
                }
            } else {
                if king_row != 0 {
                    return false;
                }

                // Check if the squares between king and rook are empty
                let (start, end) = if rook_pos.1 < king_col {
                    (rook_pos.1, king_col)
                } else {
                    (king_col, rook_pos.1)
                };

                for col in start + 1..end {
                    match *chessboard.get_square(king_row, col) {
                        Square::Empty => {}
                        Square::Occupied(_) => return false,
                    }
                }
            }

            // Check if the king is not in check during the move
            let mut temp_board = chessboard.clone();
            temp_board.update((
                king_pos.0,
                king_pos.1,
                king_pos.0,
                king_pos.1 + (rook_pos.1 - king_col),
            ));

            if temp_board.is_in_check(self.color) {
                return false;
            }

            true
        } else {
            false
        }
    }
}

impl ChessPiece for Rook {
    fn color(&self) -> Color {
        self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'R',
            Color::Black => 'r',
        }
    }

    fn valid_moves(&self, start: (usize, usize), chessboard: &Chessboard) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for dir in &directions {
            let mut x = start.0 as i32 + dir.0;
            let mut y = start.1 as i32 + dir.1;

            while x >= 0 && y >= 0 && x < 8 && y < 8 {
                match chessboard.get_square(x as usize, y as usize) {
                    Square::Empty => moves.push((x as usize, y as usize)),
                    Square::Occupied(piece) => {
                        if piece.color() != self.color() {
                            moves.push((x as usize, y as usize));
                        }
                        break;
                    }
                }

                x += dir.0;
                y += dir.1;
            }
        }

        // Add castling moves if applicable
        if self.can_castle(chessboard, start) {
            if start.1 == 0 {
                moves.push((start.0, 2));
            } else if start.1 == 7 {
                moves.push((start.0, 6));
            }
        }

        moves
    }
}
