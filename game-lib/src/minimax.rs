use std::cmp;

use crate::{
    board::{GameBoard, GameMove, GamePlayer},
    GameError,
};

pub fn minimax(mut board: GameBoard, depth: u8) -> Result<GameMove, GameError> {
    if board.is_over() {
        return Err("Game is already over");
    }      

    if depth < 1 {
        return Err("Depth must be at least 1");
    }

    let player = board.current_player();

    match player {
        GamePlayer::Opponent => {
            let mut min = i32::MAX;
            let mut min_move = None;
            for possible_move in board.moves() {
                board.make_move(&possible_move)?;

                let move_score = max_player(&mut board, i32::MIN, min, depth - 1)?;
                if move_score < min {
                    min = move_score;
                    min_move = Some(possible_move);
                }
                board.undo_move(&possible_move)?;
            }

            min_move
        }
        GamePlayer::Computer => {
            let mut max = i32::MIN;
            let mut max_move = None;
            for possible_move in board.moves() {
                board.make_move(&possible_move)?;

                let move_score = min_player(&mut board, max, i32::MAX, depth - 1)?;
                if move_score > max {
                    max = move_score;
                    max_move = Some(possible_move);
                }

                board.undo_move(&possible_move)?;
            }

            max_move
        }
    }
    .ok_or("Internal Error in minimax")
}

fn min_player(
    board: &mut GameBoard,
    alpha: i32,
    mut beta: i32,
    depth: u8,
) -> Result<i32, GameError> {
    if depth == 0 || board.is_over() {
        return Ok(board.current_score());
    }

    let mut min = i32::MAX;
    for possible_move in board.moves() {
        board.make_move(&possible_move)?;

        min = cmp::min(max_player(board, alpha, beta, depth - 1)?, min);
        beta = cmp::min(beta, min);
        board.undo_move(&possible_move)?;
        if alpha >= min {
            break;
        }
    }

    Ok(min)
}

fn max_player(
    board: &mut GameBoard,
    mut alpha: i32,
    beta: i32,
    depth: u8,
) -> Result<i32, GameError> {
    if depth == 0 || board.is_over() {
        return Ok(board.current_score());
    }

    let mut max = i32::MIN;
    for possible_move in board.moves() {
        board.make_move(&possible_move)?;
        max = cmp::max(min_player(board, alpha, beta, depth - 1)?, max);
        alpha = cmp::max(max, alpha);
        board.undo_move(&possible_move)?;
        if max >= beta {
            break;
        }
    }

    return Ok(max);
}
