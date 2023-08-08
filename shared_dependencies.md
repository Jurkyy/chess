Shared Dependencies:

1. "Color" Enum: Used in "king.rs", "rook.rs", "pawn.rs", "bishop.rs", "knight.rs", and "piece.rs" to denote the color of the chess pieces.

2. "Piece" Struct: Used in "board.rs", "game.rs", "move.rs", "castling.rs", and all the specific piece files ("king.rs", "rook.rs", etc.) to represent a chess piece.

3. "Board" Struct: Used in "game.rs", "move.rs", "castling.rs", and "piece.rs" to represent the chess board.

4. "is_valid_move" Function: Used in "king.rs", "rook.rs", "pawn.rs", "bishop.rs", "knight.rs", and "piece.rs" to check if a move is valid.

5. "castling" Function: Used in "game.rs", "move.rs", and "castling.rs" to implement the castling move.

6. "en_passant" Function: Used in "pawn.rs" and "move.rs" to implement the en passant move.

7. "two_tile_move" Function: Used in "pawn.rs" and "move.rs" to implement the two tile move for pawns.

8. "rook_move" Function: Used in "rook.rs" and "move.rs" to implement the rook's movement.

9. "knight_move" Function: Used in "knight.rs" and "move.rs" to implement the knight's movement.

10. "bishop_move" Function: Used in "bishop.rs" and "move.rs" to implement the bishop's movement.

11. Test Functions: Used in all the "tests" files to test the functionality of the corresponding files.

12. "Game" Struct: Used in "game.rs" and "castling.rs" to represent the state of the game.

13. "Move" Struct: Used in "move.rs", "game.rs", and "castling.rs" to represent a move in the game.