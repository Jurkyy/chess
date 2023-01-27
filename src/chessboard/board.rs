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

    Ok((rank, file))
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
    has_moved: bool,
}

impl Piece {
    fn new(piece_type: PieceType, color: Color, has_moved: bool) -> Piece {
        Piece {
            piece_type,
            color,
            has_moved,
        }
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
enum Direction {
    Vertical,
    Horizontal,
    Diagonal,
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
            board.squares[1][i] = Some(Piece::new(PieceType::Pawn, Color::White, false));
            board.squares[6][i] = Some(Piece::new(PieceType::Pawn, Color::Black, false));
        }

        // Initialize the rooks
        board.squares[0][0] = Some(Piece::new(PieceType::Rook, Color::White, false));
        board.squares[0][7] = Some(Piece::new(PieceType::Rook, Color::White, false));
        board.squares[7][0] = Some(Piece::new(PieceType::Rook, Color::Black, false));
        board.squares[7][7] = Some(Piece::new(PieceType::Rook, Color::Black, false));

        // Initialize the knights
        board.squares[0][1] = Some(Piece::new(PieceType::Knight, Color::White, false));
        board.squares[0][6] = Some(Piece::new(PieceType::Knight, Color::White, false));
        board.squares[7][1] = Some(Piece::new(PieceType::Knight, Color::Black, false));
        board.squares[7][6] = Some(Piece::new(PieceType::Knight, Color::Black, false));

        // Initialize the bishops
        board.squares[0][2] = Some(Piece::new(PieceType::Bishop, Color::White, false));
        board.squares[0][5] = Some(Piece::new(PieceType::Bishop, Color::White, false));
        board.squares[7][2] = Some(Piece::new(PieceType::Bishop, Color::Black, false));
        board.squares[7][5] = Some(Piece::new(PieceType::Bishop, Color::Black, false));

        // Initialize the queen and king
        board.squares[0][3] = Some(Piece::new(PieceType::Queen, Color::White, false));
        board.squares[0][4] = Some(Piece::new(PieceType::King, Color::White, false));
        board.squares[7][3] = Some(Piece::new(PieceType::Queen, Color::Black, false));
        board.squares[7][4] = Some(Piece::new(PieceType::King, Color::Black, false));

        board
    }

    pub fn print_chessboard(&self) {
        for i in (0..8).rev() {
            for j in 0..8 {
                let piece = self.squares[i][j];
                // print!("{}{} ", i, j);
                if let Some(p) = piece {
                    print!("{} ", p.fen_char());
                } else {
                    print!("_ ");
                }
            }
            println!("");
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
        match self.move_piece(origin, destination, false) {
            Ok(()) => println!("Success!"),
            Err(e) => println!("{}", e),
        }
    }

    fn move_piece(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        ignore_ownership: bool,
    ) -> Result<(), &'static str> {
        // Expand tuples
        let (f_x, f_y) = from;
        let (t_x, t_y) = to;

        // Check if piece exists
        if let Some(piece) = &self.squares[f_x][f_y] {
            println!("Moving piece {:?} from {:?} to {:?}", piece, from, to);

            if !ignore_ownership && piece.color != self.turn {
                return Err("You can only move your own pieces.");
            }
            // Check if move is valid
            else if !self.valid_move(from, to, *piece) {
                return Err("Illegal move.");
            }
            // Legal move
            self.squares[t_x][t_y] = Some(*piece);
            self.squares[f_x][f_y] = None;
        } else {
            return Err("This board cell is empty.");
        }
        Ok(())
    }

    fn valid_move(&self, from: (usize, usize), to: (usize, usize), piece: Piece) -> bool {
        match piece.piece_type {
            PieceType::Pawn => self.pawn_move(from, to, piece),
            PieceType::Bishop => self.bishop_move(from, to, piece),
            PieceType::Knight => self.knight_move(from, to, piece),
            PieceType::Rook => self.rook_move(from, to, piece),
            PieceType::Queen => self.queen_move(from, to, piece),
            PieceType::King => self.king_move(from, to, piece),
        }
    }

    fn move_vertical(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        from.0 == to.0 && self.clear_way(Direction::Vertical, from, to)
    }

    fn move_horizontal(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        from.1 == to.1 && self.clear_way(Direction::Horizontal, from, to)
    }

    fn move_diagonal(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        (0..8).any(|x| {
            // right and up or down
            ((from.0 + x == to.0 &&
            (from.1 + x == to.1 || from.1.checked_sub(x).is_some() && from.1 - x == to.1)) ||
            // left and up or down
            (from.0.checked_sub(x).is_some() && from.0 - x == to.0 &&
            (from.1 + x == to.1 || from.1.checked_sub(x).is_some() && from.1 - x == to.1)))
        }) && self.clear_way(Direction::Diagonal, from, to)
    }

    fn clear_way(&self, direction: Direction, from: (usize, usize), to: (usize, usize)) -> bool {
        match direction {
            Direction::Vertical => {
                if to.1 > from.1 {
                    (from.1 + 1..to.1).all(|x| self.squares[from.0][x].is_none())
                } else if to.1 < from.1 {
                    (to.1 + 1..from.1).all(|x| self.squares[from.0][x].is_none())
                } else {
                    false
                }
            }
            Direction::Horizontal => {
                if to.0 > from.0 {
                    (from.0 + 1..to.0).all(|x| self.squares[x][from.1].is_none())
                } else if to.0 < from.0 {
                    (to.0 + 1..from.0).all(|x| self.squares[x][from.1].is_none())
                } else {
                    false
                }
            }
            Direction::Diagonal => {
                if to.0 > from.0 && to.1 > from.1 {
                    let diff = to.0 - from.0;
                    (1..diff).all(|x| self.squares[from.0 + x][from.1 + x].is_none())
                } else if to.0 > from.0 && to.1 < from.1 {
                    let diff = to.0 - from.0;
                    (1..diff).all(|x| {
                        from.1.checked_sub(x).is_some()
                            && self.squares[from.0 + x][from.1 - x].is_none()
                    })
                } else if to.0 < from.0 && to.1 > from.1 {
                    let diff = from.0 - to.0;
                    (1..diff).all(|x| {
                        from.0.checked_sub(x).is_some()
                            && self.squares[from.0 - x][from.1 + x].is_none()
                    })
                } else if to.0 < from.0 && to.1 < from.1 {
                    let diff = from.0 - to.0;
                    (1..diff).all(|x| {
                        from.0.checked_sub(x).is_some()
                            && self.squares[from.0 - x][from.1 - x].is_none()
                    })
                } else {
                    false
                }
            }
        }
    }

    fn pawn_move(&self, from: (usize, usize), to: (usize, usize), piece: Piece) -> bool {
        let (f_x, f_y) = from;
        let (t_x, t_y) = to;

        println!(
            "from: {:?} we have {:?}  and  to: {:?} we have {:?}",
            from, self.squares[f_x][f_y], to, self.squares[t_x][t_y]
        );
        let dx: i8 = i8::abs(f_y as i8 - t_y as i8);
        let dy: i8 = i8::abs(f_x as i8 - t_x as i8);
        let move_forward;
        let move_diag;

        // Check if the pawn is moving in the correct direction
        if piece.color == Color::White {
            move_forward = dy == 1 && dx == 0;
            move_diag = dy == 1 && (dx == 1 || dx == -1);
        } else {
            move_forward = dy == -1 && dx == 0;
            move_diag = dy == -1 && (dx == 1 || dx == -1);
        }

        // Check if the pawn is moving forward
        if move_forward {
            // Check if the destination cell is empty
            return self.squares[t_x][t_y].is_none();
        }
        // Check if the pawn is moving diagonally
        else if move_diag {
            // Check if the destination cell is occupied by an enemy piece
            if let Some(pos_piece) = self.squares[t_x][t_y] {
                return pos_piece.color != piece.color;
            }
        }
        // Check if the pawn is moving two squares forward
        else if dx == 0 && dy == 2 && !piece.has_moved {
            // Check if the destination cell is empty
            return self.squares[t_x][t_y].is_none();
        }

        false
    }

    fn rook_move(&self, from: (usize, usize), to: (usize, usize), piece: Piece) -> bool {
        let mut to_color = true;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
        }
        let vertical_movement = self.move_vertical(from, to);
        let horizontal_movement = self.move_horizontal(from, to);

        println!("{} {} {}", to_color, vertical_movement, horizontal_movement);
        to_color
            && ((vertical_movement && !horizontal_movement)
                || (!vertical_movement && horizontal_movement))
    }

    fn knight_move(&self, from: (usize, usize), to: (usize, usize), piece: Piece) -> bool {
        let mut to_color = true;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
        }

        let vertical = {
            if from.1 + 2 == to.1 || (from.1.checked_sub(2).is_some() && from.1 - 2 == to.1) {
                from.0 + 1 == to.0 || (from.0.checked_sub(1).is_some() && from.0 - 1 == to.0)
            } else if from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() && from.1 - 1 == to.1)
            {
                from.0 + 2 == to.0 || (from.0.checked_sub(2).is_some() && from.0 - 2 == to.0)
            } else {
                false
            }
        };

        let horizontal = {
            if from.0 + 2 == to.0 || (from.0.checked_sub(2).is_some() && from.0 - 2 == to.0) {
                from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() && from.1 - 1 == to.1)
            } else if from.0 + 1 == to.0 || (from.0.checked_sub(1).is_some() && from.0 - 1 == to.0)
            {
                from.1 + 2 == to.1 || (from.1.checked_sub(2).is_some() && from.1 - 2 == to.1)
            } else {
                false
            }
        };

        to_color && (vertical || horizontal)
    }

    fn bishop_move(&self, from: (usize, usize), to: (usize, usize), piece: Piece) -> bool {
        let mut to_color = true;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
        }

        to_color && self.move_diagonal(from, to)
    }

    fn queen_move(&self, from: (usize, usize), to: (usize, usize), piece: Piece) -> bool {
        let mut to_color = true;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
        }

        let vertical_movement = self.move_vertical(from, to);
        let horizontal_movement = self.move_horizontal(from, to);
        let diagonal_movement = self.move_diagonal(from, to);

        (vertical_movement && !horizontal_movement && !diagonal_movement && to_color)
            || (!vertical_movement && horizontal_movement && !diagonal_movement && to_color)
            || (!vertical_movement && !horizontal_movement && diagonal_movement && to_color)
    }
    fn king_move(&self, from: (usize, usize), to: (usize, usize), piece: Piece) -> bool {
        let mut to_color = true;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
        }

        let direction = {
            // forward or backward
            if from.0 == to.0 {
                from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() && from.1 - 1 == to.1)
            // right or left
            } else if from.1 == to.1 {
                from.0 + 1 == to.0 || (from.0.checked_sub(1).is_some() && from.0 - 1 == to.0)
            // diagonal right
            } else if from.0 + 1 == to.0 {
                from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() && from.1 - 1 == to.1)
            // diagonal left
            } else if from.0.checked_sub(1).is_some() && from.0 - 1 == to.0 {
                from.0 + 1 == to.0 || (from.0.checked_sub(1).is_some() && from.0 - 1 == to.0)
            } else {
                false
            }
        };

        to_color && direction
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
