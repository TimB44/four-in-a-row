use std::cmp::{max, min, min_by, max_by};

use crate::{GameError, board::{GameBoard, GameMove, GamePlayer}};

pub fn minimax(board:&mut GameBoard, depth: u8) -> Result<GameMove, GameError> {
    if board.is_over() {
        return Err("Game is already over");
    }

    match board.current_player() {
        GamePlayer::Computer => {
            let max_so_far = i32::MIN;
            let mut max_move = None;
            for possible_move in board.moves() {
                board.make_move(&possible_move)?;
                if min_player(&mut board, 0, 0, depth)? > max_so_far {
                    max_move = Some(possible_move);
                }
            }

            if max_move.is_none() {
                return Err("No possible moves, however game is not over")
            }
            Ok(max_move.unwrap())
        },
        GamePlayer::Opponent => {
            let min_so_far = i32::MAX;
            let mut min_move = None;
            for possible_move in board.moves() {
                board.make_move(&possible_move)?;
                if min_player(&mut board, 0, 0, depth)? < min_so_far {
                    min_move = Some(possible_move);
                }
                board.undo_move(&possible_move)?;
            }

            if min_move.is_none() {
                return Err("No possible moves, however game is not over")
            }
            Ok(min_move.unwrap())
        }
    }
}

fn min_player(board: &mut GameBoard, mut alpha: i32, mut beta: i32, depth: u8) -> Result<(i32, Option<GameMove>), GameError> {
    if depth == 0 {
        return Ok((board.current_score(), None));
    }

    let mut min_move = (i32::MAX, None);
    for possible_move in board.moves() {
        board.make_move(&possible_move)?;
        min_move = min_by(min_move, max_player(board, alpha, beta, depth - 1)?, |lhs, rhs| lhs.0.cmp(&rhs.0));
        if alpha >= beta {
            break;
        }
    }

    return Ok(min_move);
}

fn max_player(board: &mut GameBoard, mut alpha: i32, mut beta: i32, depth: u8) -> Result<(i32, Option<GameMove>), GameError> {
    if depth == 0 {
        return Ok((board.current_score(), None));
    }

    let max_move  = (i32::MIN, None);
    for possible_moves in board.moves() {
        board.make_move(&possible_moves)?;
        let max_move = max_by(max_move, min_player(board, alpha, beta, depth - 1)?, |lhs, rhs| lhs.0.cmp(&rhs.0));
        alpha  = max(max_move.0, alpha);
        if alpha >= beta {
            break;
        }
    }

    return Ok(max_move);

}