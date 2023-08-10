use crate::game_components::chessboard::Chessboard;
use crate::player_management::player::Player;

pub enum Ending {
    Victory,
    Tie,
}

pub fn check_winning_condition<'a>(
    player: &'a Player,
    chessboard: &'a Chessboard,
) -> Option<(Ending, &'a Player)> {
    if checkmate(player, chessboard) {
        return Some((Ending::Victory, player));
    } else if stalemate(player, chessboard) {
        return Some((Ending::Tie, player));
    }
    None
}

fn checkmate(player: &Player, chessboard: &Chessboard) -> bool {
    if !chessboard.is_in_check(*player.color()) {
        return false;
    }

    let all_moves = chessboard.generate_all_moves(*player.color());

    for temp_move in all_moves {
        let mut temp_board = chessboard.clone();
        temp_board.update(temp_move);

        if !temp_board.is_in_check(*player.color()) {
            return false;
        }
    }

    true
}

fn stalemate(player: &Player, chessboard: &Chessboard) -> bool {
    if chessboard.is_in_check(*player.color()) {
        return false;
    }

    let all_moves = chessboard.generate_all_moves(*player.color());

    for temp_move in all_moves {
        let mut temp_board = chessboard.clone();
        temp_board.update(temp_move);

        if !temp_board.is_in_check(*player.color()) {
            return false;
        }
    }

    true
}
