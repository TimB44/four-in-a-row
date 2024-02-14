use std::cmp::max;

use crate::{GameError, board::{GameBoard, GameMove, GamePlayer}};

/// Simple algorithm which Searches for possible wins, or blocks possible losses.
/// If neither occur then it will place semi-randomly 
pub fn play_easy(board: &GameBoard) -> Result<GameMove, GameError>  {
    if board.is_over() {
        return Err("Game is already over")
    }
    if board.current_player() == GamePlayer::Computer {

    }
    let mut max_score = i32::MIN;
    let mut max_move: GameMove;

    for game_move in board.moves() {
        board.make_move(&game_move)?;
        if board.is_over(){
            board.undo_move(&game_move)?;
            return Ok(game_move);
        }
        let new_score = board.current_score();
        if new_score > max_score {
            max_score = new_score;
            max_move = game_move;

        }
        max_score = max(max_score, board.current_score());
        board.undo_move(&game_move)?;
    }

    Ok(max_move)
}
