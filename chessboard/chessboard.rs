#[derive(Copy, Clone, PartialEq)]
enum Piece {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(PartialEq, Copy, Clone)]
enum Color {
    White,
    Black,
}

struct ChessBoard {
    squares: [[Piece; 8]; 8],
    turn: Color,
    selected_piece: Option<Piece>,
}

pub impl ChessBoard {
    fn new() -> ChessBoard {
        let mut board = ChessBoard {
            squares: [[Piece::None; 8]; 8],
            turn: Color::White,
            selected_piece: None,
        };

        // Initialize the pawns
        for i in 0..8 {
            board.squares[1][i] = Piece::Pawn;
            board.squares[6][i] = Piece::Pawn;
        }

        // Initialize the rooks
        board.squares[0][0] = Piece::Rook;
        board.squares[0][7] = Piece::Rook;
        board.squares[7][0] = Piece::Rook;
        board.squares[7][7] = Piece::Rook;

        // Initialize the knights
        board.squares[0][1] = Piece::Knight;
        board.squares[0][6] = Piece::Knight;
        board.squares[7][1] = Piece::Knight;
        board.squares[7][6] = Piece::Knight;

        // Initialize the bishops
        board.squares[0][2] = Piece::Bishop;
        board.squares[0][5] = Piece::Bishop;
        board.squares[7][2] = Piece::Bishop;
        board.squares[7][5] = Piece::Bishop;

        // Initialize the queen and king
        board.squares[0][3] = Piece::Queen;
        board.squares[0][4] = Piece::King;
        board.squares[7][3] = Piece::Queen;
        board.squares[7][4] = Piece::King;

        board
    }

    fn select_piece(&mut self, position: (usize, usize)) -> Option<Piece> {
        let selected_piece = self.squares[position.0][position.1];
        if selected_piece != Piece::None {
            self.selected_piece = Some(selected_piece);
            Some(selected_piece)
        } else {
            None
        }
    }

    fn move_selected_piece(&mut self, selected_pos: (usize, usize), to: (usize, usize)) -> bool {
        if self.is_valid_move(selected_pos, to) {
            self.move_piece(selected_pos, to);
            self.selected_piece = None;
            self.turn = if self.turn == Color::White {
                Color::Black
            } else {
                Color::White
            };
            true
        } else {
            false
        }
    }

    fn get_turn(&self) -> Color {
        self.turn
    }

    fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let from_piece = self.squares[from.0][from.1];
        let to_piece = self.squares[to.0][to.1];

        match from_piece {
            Piece::Pawn => {
                // Pawns can only move forward
                if from.0 > to.0 {
                    return false;
                }

                // Pawns can move one or two squares forward on their first move
                if from.0 == 1 && (to.0 - from.0) > 2 {
                    return false;
                }

                // Pawns can only move one square forward on subsequent moves
                if from.0 != 1 && (to.0 - from.0) != 1 {
                    return false;
                }

                // Pawns can only capture diagonally
                if from.1 != to.1 && to_piece == Piece::None {
                    return false;
                }
            }
            Piece::Rook => {
                // Rooks can only move horizontally or vertically
                if from.0 != to.0 && from.1 != to.1 {
                    return false;
                }
            }
            Piece::Knight => {
                // Knights move in an L shape
                let (dx, dy) = (to.0 as i8 - from.0 as i8, to.1 as i8 - from.1 as i8);
                if dx.abs() + dy.abs() != 3 || dx.abs() == 3 || dy.abs() == 3 {
                    return false;
                }
            }
            Piece::Bishop => {
                // Bishops can only move diagonally
                if (to.0 as i8 - from.0 as i8).abs() != (to.1 as i8 - from.1 as i8).abs() {
                    return false;
                }
            }
            Piece::Queen => {
                // Queen can move horizontally, vertically, or diagonally
                if from.0 != to.0
                    && from.1 != to.1
                    && (to.0 as i8 - from.0 as i8).abs() != (to.1 as i8 - from.1 as i8).abs()
                {
                    return false;
                }
            }
            Piece::King => {
                // King can move one square in any direction
                if (to.0 as i8 - from.0 as i8).abs() > 1 || (to.1 as i8 - from.1 as i8).abs() > 1 {
                    return false;
                }
            }
            Piece::None => return false,
        }

        true
    }

    fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        if !self.is_valid_move(from, to) {
            return false;
        }

        let piece = self.squares[from.0][from.1];
        self.squares[from.0][from.1] = Piece::None;
        self.squares[to.0][to.1] = piece;
        true
    }
}
