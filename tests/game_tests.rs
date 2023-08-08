```rust
#[cfg(test)]
mod game_tests {
    use super::super::src::chess::game::Game;
    use super::super::src::chess::board::Board;
    use super::super::src::chess::pieces::{King, Rook};
    use super::super::src::chess::utils::Position;

    #[test]
    fn test_move() {
        let mut game = Game::new();
        let initial_position = Position::new(1, 1);
        let target_position = Position::new(1, 2);
        game.board.move_piece(&initial_position, &target_position);
        assert_eq!(game.board.get_piece(&target_position).unwrap().get_position(), &target_position);
    }

    #[test]
    fn test_castle() {
        let mut game = Game::new();
        let king_initial_position = Position::new(4, 0);
        let rook_initial_position = Position::new(7, 0);
        let king_target_position = Position::new(6, 0);
        let rook_target_position = Position::new(5, 0);
        game.board.move_piece(&king_initial_position, &king_target_position);
        game.board.move_piece(&rook_initial_position, &rook_target_position);
        assert_eq!(game.board.get_piece(&king_target_position).unwrap().get_position(), &king_target_position);
        assert_eq!(game.board.get_piece(&rook_target_position).unwrap().get_position(), &rook_target_position);
    }

    #[test]
    fn test_check() {
        let mut game = Game::new();
        let king_position = Position::new(4, 0);
        let enemy_rook_position = Position::new(4, 7);
        game.board.move_piece(&enemy_rook_position, &Position::new(4, 1));
        assert!(game.is_check(&king_position));
    }

    #[test]
    fn test_checkmate() {
        let mut game = Game::new();
        let king_position = Position::new(4, 0);
        let enemy_rook_position = Position::new(4, 7);
        let enemy_queen_position = Position::new(3, 7);
        game.board.move_piece(&enemy_rook_position, &Position::new(4, 1));
        game.board.move_piece(&enemy_queen_position, &Position::new(5, 1));
        assert!(game.is_checkmate(&king_position));
    }
}
```