Shared Dependencies:

1. Data Schemas: 
   - Piece: Represents a chess piece, with properties for type (pawn, knight, bishop, rook, queen, king), color (white, black), and position (x, y coordinates on the board).
   - Board: Represents the chess board, with a 2D array of Pieces.
   - Game: Represents a game of chess, with properties for the current board state, the current player's turn, and the game status (ongoing, check, checkmate, stalemate).

2. Exported Variables:
   - BOARD_SIZE: Represents the size of the chess board, typically 8x8.
   - PIECES: An enumeration of the types of chess pieces (pawn, knight, bishop, rook, queen, king).

3. Function Names:
   - move_piece: Moves a piece from one position to another, used in all piece files.
   - is_valid_move: Checks if a move is valid, used in all piece files.
   - get_possible_moves: Returns a list of all possible moves for a piece, used in all piece files.
   - is_in_check: Checks if a player is in check, used in king.rs and game.rs.
   - is_in_checkmate: Checks if a player is in checkmate, used in king.rs and game.rs.
   - can_castle: Checks if castling is possible, used in king.rs, rook.rs, and castling.rs.
   - perform_castling: Performs the castling move, used in king.rs, rook.rs, and castling.rs.

4. Message Names:
   - InvalidMoveError: Error message when a move is invalid.
   - CheckmateMessage: Message when a player is in checkmate.
   - StalemateMessage: Message when the game is in stalemate.

Note: As this is a Rust program, there are no DOM elements or JavaScript functions involved.