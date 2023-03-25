use std::cmp::Ordering;

fn parse_chess_notation(notation: &str) -> Result<(usize, usize), &str> {
    //parse chess notation
    let file = match notation.chars().next() {
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

#[derive(Debug)]
enum MoveError {
    EmptySquare,
    InvalidOwnership,
    InvalidMove,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
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

    pub fn ascii_char(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => '♟',
            (PieceType::Pawn, Color::Black) => '♙',
            (PieceType::Rook, Color::White) => '♜',
            (PieceType::Rook, Color::Black) => '♖',
            (PieceType::Knight, Color::White) => '♞',
            (PieceType::Knight, Color::Black) => '♘',
            (PieceType::Bishop, Color::White) => '♝',
            (PieceType::Bishop, Color::Black) => '♗',
            (PieceType::Queen, Color::White) => '♛',
            (PieceType::Queen, Color::Black) => '♕',
            (PieceType::King, Color::White) => '♚',
            (PieceType::King, Color::Black) => '♔',
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
    white_en_passant: Option<(usize, usize)>,
    black_en_passant: Option<(usize, usize)>,
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut board = ChessBoard {
            squares: [[None; 8]; 8],
            turn: Color::White,
            black_en_passant: None,
            white_en_passant: None,
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
                    print!("{} ", p.ascii_char());
                } else {
                    print!("_ ");
                }
            }
            println!();
        }
    }

    pub fn move_piece_from_input(&mut self, input: &str) {
        //parse input string to extract piece type, origin, and destination
        let input_parts: Vec<&str> = input.split(' ').collect();
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
            Ok(Some(captured_piece)) => println!("Captured piece: {:?}", captured_piece),
            Ok(None) => println!("Successfully moved piece."),
            Err(e) => println!("{:?}", e),
        }
    }

    fn move_piece(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        ignore_ownership: bool,
    ) -> Result<Option<(usize, usize)>, MoveError> {
        let (f_x, f_y) = from;
        let (t_x, t_y) = to;

        if let Some(piece) = self.squares[f_x][f_y] {
            if !ignore_ownership && piece.color != self.turn {
                self.squares[f_x][f_y] = Some(piece);
                Err(MoveError::InvalidOwnership)
            } else {
                let (is_valid_move, captured_piece) = self.valid_move(from, to, piece);
                if !is_valid_move {
                    return Err(MoveError::InvalidMove);
                }

                if let Some((x, y)) = captured_piece {
                    self.squares[x][y] = None;
                }

                self.squares[t_x][t_y] = Some(piece);
                self.squares[f_x][f_y] = None;

                match self.turn {
                    Color::Black => self.black_en_passant = None,
                    Color::White => self.white_en_passant = None,
                }

                self.turn = self.get_turn().opposite();

                Ok(captured_piece)
            }
        } else {
            Err(MoveError::EmptySquare)
        }
    }

    fn valid_move(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        piece: Piece,
    ) -> (bool, Option<(usize, usize)>) {
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
            (from.0 + x == to.0 &&
            (from.1 + x == to.1 || from.1.checked_sub(x).is_some() && from.1 - x == to.1)) ||
            // left and up or down
            (from.0.checked_sub(x).is_some() && from.0 - x == to.0 &&
            (from.1 + x == to.1 || from.1.checked_sub(x).is_some() && from.1 - x == to.1))
        }) && self.clear_way(Direction::Diagonal, from, to)
    }

    fn clear_way(&self, direction: Direction, from: (usize, usize), to: (usize, usize)) -> bool {
        match direction {
            Direction::Vertical => match to.1.cmp(&from.1) {
                Ordering::Greater => (from.1 + 1..to.1).all(|x| self.squares[from.0][x].is_none()),
                Ordering::Less => (to.1 + 1..from.1).all(|x| self.squares[from.0][x].is_none()),
                Ordering::Equal => false,
            },
            Direction::Horizontal => match to.0.cmp(&from.0) {
                Ordering::Greater => (from.0 + 1..to.0).all(|x| self.squares[x][from.1].is_none()),
                Ordering::Less => (to.0 + 1..from.0).all(|x| self.squares[x][from.1].is_none()),
                Ordering::Equal => false,
            },
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

    fn pawn_move(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        piece: Piece,
    ) -> (bool, Option<(usize, usize)>) {
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
            return (self.squares[t_x][t_y].is_none(), None);
        }
        // Check if the pawn is moving diagonally
        else if move_diag {
            // Check if the destination cell is occupied by an enemy piece
            if let Some(pos_piece) = self.squares[t_x][t_y] {
                return (pos_piece.color != piece.color, Some(to));
            }
            // Check for en passant move
            if let Some(en_passant) = if piece.color == Color::White {
                self.white_en_passant
            } else {
                self.black_en_passant
            } {
                // Check if the destination cell matches the en passant square
                if en_passant == to {
                    // Cast to isize so we can subtract.
                    let enemy_pawn_pos = (
                        t_x as isize - if piece.color == Color::White { 1 } else { -1 },
                        t_y as isize,
                    );
                    let enemy_pawn_pos = (enemy_pawn_pos.0 as usize, enemy_pawn_pos.1 as usize);
                    // Check if there actually is a pawn and return true if so.
                    if let Some(enemy_pawn) = self.squares[enemy_pawn_pos.0][enemy_pawn_pos.1] {
                        if enemy_pawn.piece_type == PieceType::Pawn
                            && enemy_pawn.color != piece.color
                        {
                            return (true, Some(enemy_pawn_pos));
                        }
                    }
                }
            }
        }
        // Check if the pawn is moving two squares forward
        else if dx == 0 && dy == 2 && !piece.has_moved {
            // Set the en passant field correctly.
            if piece.color == Color::White {
                self.black_en_passant = Some((f_x + 1, f_y));
            } else {
                self.white_en_passant = Some((f_x - 1, f_y));
            }
            // Check if the destination cell is empty
            return (self.squares[t_x][t_y].is_none(), None);
        }

        (false, None)
    }

    fn rook_move(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        piece: Piece,
    ) -> (bool, Option<(usize, usize)>) {
        let mut to_color = true;
        let mut taken_position: Option<(usize, usize)> = None;

        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
            taken_position = Some(to);
        }
        let vertical_movement = self.move_vertical(from, to);
        let horizontal_movement = self.move_horizontal(from, to);

        println!("{} {} {}", to_color, vertical_movement, horizontal_movement);
        (
            to_color
                && ((vertical_movement && !horizontal_movement)
                    || (!vertical_movement && horizontal_movement)),
            taken_position,
        )
    }

    fn knight_move(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        piece: Piece,
    ) -> (bool, Option<(usize, usize)>) {
        let mut to_color = true;
        let mut taken_position: Option<(usize, usize)> = None;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
            taken_position = Some(to);
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

        (to_color && (vertical || horizontal), taken_position)
    }

    fn bishop_move(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        piece: Piece,
    ) -> (bool, Option<(usize, usize)>) {
        let mut to_color = true;
        let mut taken_position: Option<(usize, usize)> = None;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
            taken_position = Some(to);
        }

        (to_color && self.move_diagonal(from, to), taken_position)
    }

    fn queen_move(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        piece: Piece,
    ) -> (bool, Option<(usize, usize)>) {
        let mut to_color = true;
        let mut taken_position: Option<(usize, usize)> = None;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
            taken_position = Some(to);
        }

        let vertical_movement = self.move_vertical(from, to);
        let horizontal_movement = self.move_horizontal(from, to);
        let diagonal_movement = self.move_diagonal(from, to);

        (
            to_color
                && ((vertical_movement as u8
                    + horizontal_movement as u8
                    + diagonal_movement as u8)
                    == 1),
            taken_position,
        )
        // Are these equivalent?
        //  (vertical_movement && !horizontal_movement && !diagonal_movement && to_color)
        //     || (!vertical_movement && horizontal_movement && !diagonal_movement && to_color)
        //     || (!vertical_movement && !horizontal_movement && diagonal_movement && to_color)
    }

    fn king_move(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        piece: Piece,
    ) -> (bool, Option<(usize, usize)>) {
        let mut to_color = true;
        let mut taken_position: Option<(usize, usize)> = None;
        if let Some(temp) = self.squares[to.0][to.1] {
            to_color = temp.color != piece.color;
            taken_position = Some(to);
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

        (to_color && direction, taken_position)
    }

    fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        if x < self.squares.len() && y < self.squares[x].len() {
            return self.squares[x][y];
        }
        None
    }

    pub fn fen(&self) -> String {
        // Piece placement data
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
            }
            if i < 7 {
                fen_string.push('/');
            }
        }

        // Add active color
        let turn = self.get_turn();
        if turn == Color::White {
            fen_string.push_str(" w");
        } else {
            fen_string.push_str(" b");
        }

        // Enconde the castling rights
        fen_string.push(' ');

        // Encode the en passant target square(s)
        fen_string.push(' ');
        if let Some((rank, file)) = self.white_en_passant {
            fen_string.push_str(&format!("{}{}", char::from(b'a' + file as u8), rank + 1));
        } else if let Some((rank, file)) = self.black_en_passant {
            fen_string.push_str(&format!("{}{}", char::from(b'a' + file as u8), rank + 1));
        } else {
            fen_string.push('-');
        }

        // Enconde the halfmove and fullmove clocks
        fen_string.push(' ');

        println!("FEN: {}", fen_string);
        fen_string
    }
}
