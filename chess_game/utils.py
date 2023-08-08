def is_valid_castling(chess_board, chess_piece, target_position):
    """
    Check if the castling move is valid.
    """
    # Check if the piece is a king and it's the first move
    if chess_piece.type != 'King' or not chess_piece.is_first_move:
        return False

    # Check if the target position is valid for castling
    if target_position not in [(0, 2), (0, 6), (7, 2), (7, 6)]:
        return False

    # Check if the path between the king and the rook is clear
    path_is_clear = chess_board.is_path_clear(chess_piece.position, target_position)
    if not path_is_clear:
        return False

    # Check if the king is under attack in the current, intermediate or final position
    if chess_board.is_under_attack(chess_piece.position) or \
       chess_board.is_under_attack(chess_piece.position[0], target_position[1] - 1) or \
       chess_board.is_under_attack(target_position):
        return False

    return True

def execute_castling(chess_board, chess_piece, target_position):
    """
    Execute the castling move.
    """
    # Move the king
    chess_board.move_piece(chess_piece, target_position)

    # Move the rook
    rook_position = (target_position[0], 0 if target_position[1] == 2 else 7)
    new_rook_position = (target_position[0], 3 if target_position[1] == 2 else 5)
    rook = chess_board.get_piece(rook_position)
    chess_board.move_piece(rook, new_rook_position)