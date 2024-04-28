//! #  Connect 4 Game Library
//!
//! `game-lib` provides utilities for the connect 4 games, and includes ai
//!  algorithms to provide varying difficulties of opponents.
//!

use board::{GameBoard, GamePlayer};

mod ai_players;
pub mod board;
mod minimax;

type GameError = &'static str;

/// Calculates the next move in the given board by preforming a shallow search
/// of the Game Tree. The Given board must be a valid connect four game that
/// is still in progress.
///
/// # Format of The Board
/// The board is a 2d array of signed bytes. The outer array holds the array of rows.
/// the first row is `board[0]`
///
/// The pieces are represented as follows
/// - O represents an empty spot
/// - 1 represents a piece from the first player
/// - -1 represents a piece from the second player
///
/// # Errors
/// This function will return a string error if the given board is not a valid
/// in progress connect 4 game. Below are some ways a board can be invalid
/// - The Game is already over ei. Tie or 4 in a row
/// - The Board contains floating pieces
/// - The Board contains an invalid number of either players piece
///
pub fn next_easy_move(board: [[i8; 7]; 6]) -> Result<u8, GameError> {
    let board = parse_board(board)?;

    Ok(ai_players::play_easy(board)?.get_col())
}

/// Calculates the next move in the given board by preforming a search
/// of the Game Tree. The Given board must be a valid connect four game that
/// is still in progress.
///
/// # Format of The Board
/// The board is a 2d array of signed bytes. The outer array holds the array of rows.
/// the first row is `board[0]`
///
/// The pieces are represented as follows
/// - O represents an empty spot
/// - 1 represents a piece from the first player
/// - -1 represents a piece from the second player
///
/// # Errors
/// This function will return a string error if the given board is not a valid
/// in progress connect 4 game. Below are some ways a board can be invalid
/// - The Game is already over ei. Tie or 4 in a row
/// - The Board contains floating pieces
/// - The Board contains an invalid number of either players piece
///
pub fn next_medium_move(board: [[i8; 7]; 6]) -> Result<u8, GameError> {
    let board = parse_board(board)?;

    Ok(ai_players::play_med(board)?.get_col())
}

/// Calculates the next move in the given board by preforming a search
/// of the Game Tree. The Given board must be a valid connect four game that
/// is still in progress.
///
/// # Format of The Board
/// The board is a 2d array of signed bytes. The outer array holds the array of rows.
/// the first row is `board[0]`
///
/// The pieces are represented as follows
/// - O represents an empty spot
/// - 1 represents a piece from the first player
/// - -1 represents a piece from the second player
///
/// # Errors
/// This function will return a string error if the given board is not a valid
/// in progress connect 4 game. Below are some ways a board can be invalid
/// - The Game is already over ei. Tie or 4 in a row
/// - The Board contains floating pieces
/// - The Board contains an invalid number of either players piece
///
pub fn next_hard_move(board: [[i8; 7]; 6]) -> Result<u8, GameError> {
    let board = parse_board(board)?;

    Ok(ai_players::play_hard(board)?.get_col())
}

fn parse_board(num_board: [[i8; 7]; 6]) -> Result<GameBoard, GameError> {
    let mut board = [[None; 7]; 6];

    for row in 0..6 {
        for col in 0..7 {
            board[row][col] = match num_board[row][col] {
                1 => Some(GamePlayer::FirstPlayer),
                0 => None,
                -1 => Some(GamePlayer::SecondPlayer),
                _ => return Err("Board can only contain 1, 0, and -1"),
            }
        }
    }

    GameBoard::build(board)
}
