use crate::game_components::chess_piece::ChessPiece;
use crate::game_components::chessboard::{Chessboard, Color, Square};

pub struct Bishop {
    color: Color,
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Bishop { color }
    }
}

impl ChessPiece for Bishop {
    fn color(&self) -> Color {
        self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'B',
            Color::Black => 'b',
        }
    }

    fn valid_moves(&self, start: (usize, usize), chessboard: &Chessboard) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

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

        moves
    }
}
