fn parse_chess_notation(notation: &str) -> Result<(usize, usize), &str> {
    //parse chess notation
    let file = match notation.chars().nth(0) {
        Some(c) => match c.to_ascii_lowercase() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err("Invalid chess notation"),
        },
        None => return Err("Invalid chess notation"),
    };
    let rank = match notation[1..].parse::<usize>() {
        Ok(r) => r - 1,
        Err(_) => return Err("Invalid chess notation"),
    };

    // println!("file {} and rank {}", file + 1, rank + 1);

    Ok((rank, file))
}

#[derive(Copy, Clone, PartialEq)]
enum PieceType {
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

#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    fn new(piece_type: PieceType, color: Color) -> Piece {
        let piece = Piece { piece_type, color };
        piece
    }

    fn fen_char(&self) -> char {
        match (self.color, self.piece_type) {
            (Color::White, PieceType::Pawn) => 'P',
            (Color::White, PieceType::Rook) => 'R',
            (Color::White, PieceType::Knight) => 'N',
            (Color::White, PieceType::Bishop) => 'B',
            (Color::White, PieceType::Queen) => 'Q',
            (Color::White, PieceType::King) => 'K',
            (Color::Black, PieceType::Pawn) => 'p',
            (Color::Black, PieceType::Rook) => 'r',
            (Color::Black, PieceType::Knight) => 'n',
            (Color::Black, PieceType::Bishop) => 'b',
            (Color::Black, PieceType::Queen) => 'q',
            (Color::Black, PieceType::King) => 'k',
        }
    }
}
pub struct ChessBoard {
    pub squares: [[Option<Piece>; 8]; 8],
    turn: Color,
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut board = ChessBoard {
            squares: [[None; 8]; 8],
            turn: Color::White,
        };

        // Initialize the pawns
        for i in 0..8 {
            board.squares[1][i] = Some(Piece::new(PieceType::Pawn, Color::White));
            board.squares[6][i] = Some(Piece::new(PieceType::Pawn, Color::Black));
        }

        // Initialize the rooks
        board.squares[0][0] = Some(Piece::new(PieceType::Rook, Color::White));
        board.squares[0][7] = Some(Piece::new(PieceType::Rook, Color::White));
        board.squares[7][0] = Some(Piece::new(PieceType::Rook, Color::Black));
        board.squares[7][7] = Some(Piece::new(PieceType::Rook, Color::Black));

        // Initialize the knights
        board.squares[0][1] = Some(Piece::new(PieceType::Knight, Color::White));
        board.squares[0][6] = Some(Piece::new(PieceType::Knight, Color::White));
        board.squares[7][1] = Some(Piece::new(PieceType::Knight, Color::Black));
        board.squares[7][6] = Some(Piece::new(PieceType::Knight, Color::Black));

        // Initialize the bishops
        board.squares[0][2] = Some(Piece::new(PieceType::Bishop, Color::White));
        board.squares[0][5] = Some(Piece::new(PieceType::Bishop, Color::White));
        board.squares[7][2] = Some(Piece::new(PieceType::Bishop, Color::Black));
        board.squares[7][5] = Some(Piece::new(PieceType::Bishop, Color::Black));

        // Initialize the queen and king
        board.squares[0][3] = Some(Piece::new(PieceType::Queen, Color::White));
        board.squares[0][4] = Some(Piece::new(PieceType::King, Color::White));
        board.squares[7][3] = Some(Piece::new(PieceType::Queen, Color::Black));
        board.squares[7][4] = Some(Piece::new(PieceType::King, Color::Black));

        board
    }

    pub fn print_chessboard(&self) {
        for i in (0..8).rev() {
            for j in (0..8) {
                let piece = self.squares[i][j];
                // print!("{}{} ", i, j)
                if let Some(p) = piece {
                    print!("{} ", p.fen_char());
                } else {
                    print!("_ ");
                }
            }
            println!("");
        }
    }

    fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        if self.is_valid_move(from, to) {
            let piece = self.squares[from.0][from.1];
            self.squares[from.0][from.1] = None;
            self.squares[to.0][to.1] = piece;
            self.turn = if self.turn == Color::White {
                Color::Black
            } else {
                Color::White
            };
            println!("Succeeded to move piece from {:?} to {:?}", from, to);
            true
        } else {
            println!("Failed to move piece from {:?} to {:?}", from, to);
            false
        }
    }

    pub fn move_piece_from_input(&mut self, input: &str) {
        //parse input string to extract piece type, origin, and destination
        let input_parts: Vec<&str> = input.split(" ").collect();
        let origin = match parse_chess_notation(input_parts[0]) {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };
        let destination = match parse_chess_notation(input_parts[1]) {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        //call move_piece function
        self.move_piece(origin, destination);
    }

    fn get_turn(&self) -> Color {
        self.turn
    }

    fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        if x < self.squares.len() && y < self.squares[x].len() {
            return self.squares[x][y];
        }
        None
    }

    fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let from_piece = self.squares[from.0][from.1];
        let to_piece = self.squares[to.0][to.1];

        match from_piece {
            Some(Piece {
                piece_type: PieceType::Pawn,
                color: _,
            }) => {
                println!("Moving Pawn from {:?} to {:?}", from, to);
                // Pawns can only move forward
                if from.1 > to.1 {
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
                if from.1 != to.1 && to_piece == None {
                    return false;
                }
            }
            Some(Piece {
                piece_type: PieceType::Rook,
                color: _,
            }) => {
                println!("Moving Rook from {:?} to {:?}", from, to);
                // Rooks can only move horizontally or vertically
                if from.0 != to.0 && from.1 != to.1 {
                    return false;
                }
            }
            Some(Piece {
                piece_type: PieceType::Knight,
                color: _,
            }) => {
                println!("Moving Knight from {:?} to {:?}", from, to);
                // Knights move in an L shape
                let (dx, dy) = (to.0 as i8 - from.0 as i8, to.1 as i8 - from.1 as i8);
                if dx.abs() + dy.abs() != 3 || dx.abs() == 3 || dy.abs() == 3 {
                    return false;
                }
            }
            Some(Piece {
                piece_type: PieceType::Bishop,
                color: _,
            }) => {
                println!("Moving Bishop from {:?} to {:?}", from, to);
                // Bishops can only move diagonally
                if (to.0 as i8 - from.0 as i8).abs() != (to.1 as i8 - from.1 as i8).abs() {
                    return false;
                }
            }
            Some(Piece {
                piece_type: PieceType::Queen,
                color: _,
            }) => {
                println!("Moving Queen from {:?} to {:?}", from, to);
                // Queen can move horizontally, vertically, or diagonally
                if from.0 != to.0
                    && from.1 != to.1
                    && (to.0 as i8 - from.0 as i8).abs() != (to.1 as i8 - from.1 as i8).abs()
                {
                    return false;
                }
            }
            Some(Piece {
                piece_type: PieceType::King,
                color: _,
            }) => {
                println!("Moving King from {:?} to {:?}", from, to);
                // King can move one square in any direction
                if (to.0 as i8 - from.0 as i8).abs() > 1 || (to.1 as i8 - from.1 as i8).abs() > 1 {
                    return false;
                }
            }
            None => return false,
        }

        true
    }

    pub fn fen(&self) -> String {
        let mut fen_string = String::new();
        for i in 0..8 {
            let mut empty_count = 0;
            for j in 0..8 {
                let piece = match self.get_piece(i, j) {
                    Some(p) => p,
                    None => {
                        empty_count += 1;
                        continue;
                    }
                };
                if empty_count > 0 {
                    fen_string.push_str(&empty_count.to_string());
                    empty_count = 0;
                }
                fen_string.push(piece.fen_char());
            }
            if empty_count > 0 {
                fen_string.push_str(&empty_count.to_string());
                empty_count = 0;
            }
            if i < 7 {
                fen_string.push('/');
            }
        }
        println!("FEN: {}", fen_string);
        fen_string
    }
}
