use super::chess_piece::ChessPiece;
use super::chessboard::{Chessboard, Color, Square};

pub struct Pawn {
    color: Color,
    has_moved: bool,
}

impl Pawn {
    pub fn new(color: Color, has_moved: bool) -> Self {
        Pawn { color, has_moved }
    }
}

impl ChessPiece for Pawn {
    fn color(&self) -> Color {
        self.color
    }

    fn ascii_char(&self) -> char {
        match self.color {
            Color::White => 'P',
            Color::Black => 'p',
        }
    }

    fn valid_moves(&self, start: (usize, usize), chessboard: &Chessboard) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let forward_direction = if self.color == Color::White { 1 } else { -1 };

        // Single step forward
        let x = (start.0 as i32 + forward_direction) as usize;
        let y = start.1;
        if x < 8
            && match *chessboard.get_square(x, y) {
                Square::Empty => true,
                Square::Occupied(_) => false,
            }
        {
            moves.push((x, y));

            // Double step forward (if not moved yet)
            if !self.has_moved
                && match *chessboard.get_square(x + forward_direction as usize, y) {
                    Square::Empty => true,
                    Square::Occupied(_) => false,
                }
            {
                moves.push((x + forward_direction as usize, y));
            }
        }

        // Diagonal captures
        for &dy in [-1, 1].iter() {
            let x = (start.0 as i32 + forward_direction) as usize;
            let y = (start.1 as i32 + dy) as usize;
            if x < 8 && y < 8 {
                match chessboard.get_square(x, y) {
                    Square::Occupied(piece) if piece.color() != self.color() => {
                        moves.push((x, y));
                    }
                    _ => {
                        // Check for en passant
                        if let Some(en_passant_target) = chessboard.get_en_passant_target() {
                            if en_passant_target == (x, y) {
                                moves.push((x, y));
                            }
                        }
                    }
                }
            }
        }

        moves
    }
}
