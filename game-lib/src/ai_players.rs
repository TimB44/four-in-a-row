use crate::board::{GameBoard, GameMove, Players};

/// Simple algorithm which Searches for possible wins, or blocks possible losses.
/// If neither occur then it will place semi-randomly //TODO
pub fn play_easy(board: GameBoard) -> GameMove {
    for col in 0..=7 {
        let game_move = GameMove::build(col).unwrap();

        if board.will_cause_win(&Players::You, game_move) {
            return game_move;
        }
    }

    for col in 0..=7 {
        let game_move = GameMove::build(col).unwrap();

        if board.will_cause_win(&Players::Opp, game_move) {
            return game_move;
        }
    }

    GameMove::build(
        board.get_board()[5]
            .iter()
            .take_while(|p| p.is_some())
            .count() as u8,
    )
    .unwrap()
}
