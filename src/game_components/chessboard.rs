use super::chess_piece::ChessPiece;
use super::king::King;
use std::fmt;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}
#[derive(Clone)]
pub enum Square {
    Empty,
    Occupied(Rc<dyn ChessPiece>),
}

#[derive(Clone)]
pub struct Chessboard {
    squares: Vec<Vec<Square>>,
    en_passant_target: Option<(usize, usize)>,
}

impl Chessboard {
    pub fn new() -> Chessboard {
        Chessboard {
            squares: Self::init_empty_board(),
            en_passant_target: None,
        }
    }

    // Initialize an empty board
    fn init_empty_board() -> Vec<Vec<Square>> {
        let mut board = Vec::with_capacity(8);
        for _ in 0..8 {
            let row = vec![Square::Empty; 8];
            board.push(row);
        }
        board
    }

    pub fn place_piece(&mut self, piece: Rc<dyn ChessPiece>, x: usize, y: usize) {
        self.squares[x][y] = Square::Occupied(piece);
    }

    pub fn get_square(&self, x: usize, y: usize) -> &Square {
        &self.squares[x][y]
    }

    pub fn update(
        &mut self,
        player_move: (usize, usize, usize, usize),
    ) -> Result<(), &'static str> {
        let (x1, y1, x2, y2) = player_move;

        // Retrieve the piece from the source square
        let source_piece = match &self.squares[x1][y1] {
            Square::Occupied(rc_piece) => Rc::clone(rc_piece),
            Square::Empty => return Err("No piece at the source square."),
        };

        // Move the piece to the destination square
        self.squares[x2][y2] = Square::Occupied(source_piece);
        self.squares[x1][y1] = Square::Empty;

        Ok(())
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        // Find the king's position
        let king_pos = self.find_king(color);

        // Iterate through opponent's pieces and check if any can attack the king
        for row in 0..8 {
            for col in 0..8 {
                if let Square::Occupied(piece) = self.get_square(row, col) {
                    if piece.color() != color {
                        let moves = piece.valid_moves((row, col), self);
                        if moves.contains(&king_pos) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn find_king(&self, color: Color) -> (usize, usize) {
        // Iterate through the board to find the king's position
        for row in 0..8 {
            for col in 0..8 {
                if let Square::Occupied(piece) = self.get_square(row, col) {
                    if let Some(king) = piece.as_any().downcast_ref::<Rc<King>>() {
                        if king.color() == color && king.ascii_char() == 'K' {
                            return (row, col);
                        }
                    }
                }
            }
        }
        // Return an invalid position if king not found
        (usize::MAX, usize::MAX)
    }

    pub fn set_en_passant_target(&mut self, target: Option<(usize, usize)>) {
        self.en_passant_target = target;
    }

    pub fn get_en_passant_target(&self) -> Option<(usize, usize)> {
        self.en_passant_target
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.squares {
            for square in row {
                match square {
                    Square::Empty => write!(f, ". ")?,
                    Square::Occupied(piece) => write!(f, "{} ", piece.ascii_char())?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
