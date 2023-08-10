use crate::game_components::chess_piece::ChessPiece;

use super::chessboard::{Chessboard, Color, Square};

pub struct Knight {
    color: Color,
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Knight { color }
    }
}

impl ChessPiece for Knight {
    fn color(&self) -> Color {
        self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'N',
            Color::Black => 'n',
        }
    }

    fn valid_moves(&self, start: (usize, usize), chessboard: &Chessboard) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let moves_offsets = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        for &(dx, dy) in &moves_offsets {
            let x = start.0 as i32 + dx;
            let y = start.1 as i32 + dy;

            if x >= 0 && y >= 0 && x < 8 && y < 8 {
                match chessboard.get_square(x as usize, y as usize) {
                    Square::Empty => moves.push((x as usize, y as usize)),
                    Square::Occupied(piece) => {
                        if piece.color() != self.color() {
                            moves.push((x as usize, y as usize));
                        }
                    }
                }
            }
        }

        moves
    }
}
