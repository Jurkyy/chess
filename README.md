# Chess implementation

Implements a basic chess game. The module chessboard contains the logic of chess and allows for basic interaction with the game.

Shared Dependencies:

1. **Trait**: `ChessPiece` - This trait is shared among all the chess piece files (bishop.rs, knight.rs, queen.rs, king.rs, rook.rs, pawn.rs). It defines common methods that all chess pieces must implement.

2. **Struct**: `Color` - This struct is used in the `ChessPiece` trait and therefore shared among all the chess piece files. It is also used in the player.rs file to define the color of the player's pieces.

3. **Enum**: `Square` - This enum is defined in the chessboard.rs file and used to represent the state of each square on the chessboard. It is likely used in the move_validation.rs and piece_capturing.rs files for game logic, and in the print_chessboard.rs file for displaying the game state.

4. **Struct**: `Chessboard` - This struct is defined in the chessboard.rs file and likely used in many other files to represent the game state, including: initialize_game.rs, main_game_loop.rs, move_validation.rs, and winning_condition.rs.

5. **Struct**: `Player` - This struct is defined in the player.rs file and likely used in many other files to represent the players, including: initialize_game.rs, main_game_loop.rs, and input_handling.rs.

6. **Function**: `print_chessboard` - This function is likely defined in the print_chessboard.rs file and used in the main_game_loop.rs file to display the game state.

7. **Function**: `handle_input` - This function is likely defined in the input_handling.rs file and used in the main_game_loop.rs file to get the current player's move.

8. **Function**: `validate_move` - This function is likely defined in the move_validation.rs file and used in the main_game_loop.rs file to check if the current player's move is valid.

9. **Function**: `initialize_game` - This function is likely defined in the initialize_game.rs file and used in the main.rs file to set up the game.

10. **Function**: `main_game_loop` - This function is likely defined in the main_game_loop.rs file and used in the main.rs file to run the game.

11. **Function**: `check_winning_condition` - This function is likely defined in the winning_condition.rs file and used in the main_game_loop.rs file to determine if the game has ended.