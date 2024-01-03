use rand::Rng;

use crate::board::{GameBoard, GameMove, Players};

/// Simple algorithm which Searches for possible wins, or blocks possible losses.
/// If neither occur then it will place semi-randomly //TODO
pub fn play_easy(board: GameBoard) -> GameMove {
    for col in 0..7 {
        let game_move = GameMove::build(col).unwrap();

        if board.will_cause_win(&Players::You, game_move) {
            return game_move;
        }
    }

    for col in 0..7 {
        let game_move = GameMove::build(col).unwrap();

        if board.will_cause_win(&Players::Opp, game_move) {
            return game_move;
        }
    }

    let valid_moves: Vec<usize> = board.get_board()[5]
        .iter()
        .enumerate()
        .filter(|(_, p)| p.is_none())
        .map(|(i, _)| i)
        .collect();

    let i = rand::thread_rng().gen_range(0..valid_moves.len());

    GameMove::build(i as u8).unwrap()
}
