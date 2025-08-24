//! # Board Module
//!
//! This module contains the Structs and enums used the represent the
//! connect 4 board.

use crate::GameError;
use std::cmp::{self, max, min};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[cfg(test)]
mod game_board_tests;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

/// Represents a valid connect 4 game that could be over. This struct
/// contains many methods which allow for the game to progress though
/// playing moves.
#[derive(Debug, Clone, Copy)]
pub struct GameBoard {
    board: [[Option<GamePlayer>; BOARD_WIDTH]; BOARD_HEIGHT],
    total_moves: u8,
    winning_move: Option<GameMove>,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBoard {
    /// Returns an empty board with no pieces played
    pub fn new() -> GameBoard {
        GameBoard {
            board: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            total_moves: 0,
            winning_move: None,
        }
    }

    /// Attempts to make the given move on the board
    ///
    /// # Errors
    /// - If the game is already over
    /// - If the column the piece is attempting to be placed into is full
    pub fn make_move(&mut self, game_move: &GameMove) -> Result<(), GameError> {
        let col = game_move.0 as usize;
        if self.is_over() {
            return Err("Can not make move when game is over");
        }

        let player = self.current_player();
        for i in 0..BOARD_WIDTH {
            if self.board[i][col].is_none() {
                self.board[i][col] = Some(player);
                if self.max_streak(col, i) > 3 {
                    self.winning_move = Some(*game_move)
                }
                self.total_moves += 1;
                return Ok(());
            }
        }

        Err("Can not make move because given column is full")
    }

    /// Undoes the given move on the board
    ///
    /// # Errors
    /// - The move cannot be undone while maintaining a correct board
    /// - If the previous move caused a win, then the move being undone must be the last move
    ///
    pub fn undo_move(&mut self, game_move: &GameMove) -> Result<(), GameError> {
        let col = game_move.0;
        if self.is_over() {
            if let Some(game_move) = self.winning_move {
                if game_move.0 != col {
                    return Err(
                        "If the game is over you can only undo the winning move last played",
                    );
                }
            }
            self.winning_move = None;
        }

        for i in (0..BOARD_HEIGHT).rev() {
            if let Some(player) = self.board[i][col as usize] {
                if player == self.current_player() {
                    return Err("Attempted to remove incorrect player piece");
                }

                self.board[i][col as usize] = None;
                self.total_moves -= 1;
                return Ok(());
            }
        }
        Err("No pieces in the given column to remove")
    }

    /// Returns the player whose turn it is
    pub fn current_player(&self) -> GamePlayer {
        if self.total_moves % 2 == 0 {
            GamePlayer::FirstPlayer
        } else {
            GamePlayer::SecondPlayer
        }
    }

    /// Returns true if the game is over, false if not
    pub fn is_over(&self) -> bool {
        self.winning_move.is_some() || self.total_moves == 42
    }

    /// Return the winner if the game is over, or None if the game is in progress
    pub fn winner(&self) -> Option<GamePlayer> {
        if !self.is_over() || self.total_moves == 42 {
            return None;
        }

        Some(self.current_player().other())
    }

    /// Returns a vector with all of the valid moves
    pub fn moves(&self) -> Vec<GameMove> {
        let mut moves: Vec<GameMove> = self.board[5]
            .iter()
            .enumerate()
            .filter(|(_, spot)| !self.is_over() && spot.is_none())
            .map(|(col, _)| unsafe { GameMove::build_unchecked(col as i32) })
            .collect();
        moves.shuffle(&mut thread_rng());

        moves
    }

    fn max_streak(&self, col: usize, row: usize) -> u8 {
        let player = match self.board[row][col] {
            None => return 0,
            Some(player) => player,
        };

        // Bottom branch
        let mut streak = 1;
        for i in (0..row).rev() {
            if self.board[i][col] != Some(player) {
                break;
            }
            streak += 1;
        }

        // Upper branch
        for i in (row + 1)..BOARD_HEIGHT {
            if self.board[i][col] != Some(player) {
                break;
            }
            streak += 1;
        }

        //horizontal win
        let mut new_streak = 1;
        //left branch
        for i in (0..col).rev() {
            if self.board[row][i] != Some(player) {
                break;
            }
            new_streak += 1;
        }

        //right branch
        for i in (col + 1)..BOARD_WIDTH {
            if self.board[row][i] != Some(player) {
                break;
            }
            new_streak += 1;
        }

        streak = max(streak, new_streak);

        //bottom left to top right diagonal
        new_streak = 1;

        // left branch
        let max_offset = min(row, col);
        for i in 1..=max_offset {
            if self.board[row - i][col - i] != Some(player) {
                break;
            }
            new_streak += 1;
        }

        // right branch
        let max_offset = min(5 - row, BOARD_HEIGHT - col);
        for i in 1..=max_offset {
            if self.board[row + i][col + i] != Some(player) {
                break;
            }
            new_streak += 1;
        }

        streak = max(streak, new_streak);

        //top left to bottom right diagonal
        new_streak = 1;

        //left branch
        let max_offset = min(5 - row, col);
        for i in 1..=max_offset {
            if self.board[row + i][col - i] != Some(player) {
                break;
            }
            new_streak += 1;
        }

        // right branch
        let max_offset = min(row, BOARD_HEIGHT - col);
        for i in 1..=max_offset {
            if self.board[row - i][col + i] != Some(player) {
                break;
            }
            new_streak += 1;
        }

        max(streak, new_streak)
    }

    /// Performs a static evaluation of the board in its current state.
    /// The current strategy of doing this see how large the streak is
    /// for each possible play. If the game is over, a score of 10,000
    /// will be given to the winner. Streaks of 3 after 1 move will
    /// have an effect of 100, streaks of 2: 10, and streaks of 1: 1
    pub fn current_score(&self) -> i32 {
        if self.is_over() {
            return match self.winner() {
                Some(GamePlayer::SecondPlayer) => 10_000,
                Some(GamePlayer::FirstPlayer) => -10_000,
                None => 0,
            };
        }

        let mut total = 0;

        for col in 0..BOARD_WIDTH {
            for row in 0..BOARD_HEIGHT {
                // Consider the streak if the piece is adjacent to an empty spot
                if self.board[row][col].is_some()
                    && (self.board[cmp::min(row + 1, 5)][col].is_none()
                        || self.board[min(5, row + 1)][cmp::min(col + 1, BOARD_HEIGHT)].is_none()
                        || self.board[min(5, row + 1)][cmp::max((col as isize) - 1, 0) as usize]
                            .is_none())
                {
                    let modifier = match self.board[row][col].expect("Error in current score logic")
                    {
                        GamePlayer::FirstPlayer => -1,
                        GamePlayer::SecondPlayer => 1,
                    };
                    total += 10_i32.pow((self.max_streak(col, row)).into()) * modifier;
                }
            }
        }
        total
    }

    /// Builds a GameBoard from the given 2d Array
    ///
    /// # Errors
    /// This function will return a string error if the given board is not a valid
    /// in progress connect 4 game. Below are some ways a board can be invalid
    /// - The Game is already over ei. Tie or 4 in a row
    /// - The Board contains floating pieces
    /// - The Board contains an invalid number of either players piece
    ///
    pub fn build(
        board: [[Option<GamePlayer>; BOARD_WIDTH]; BOARD_HEIGHT],
    ) -> Result<GameBoard, GameError> {
        //check for invalid number of pieces
        let mut first_count: i32 = 0;
        let mut second_count: i32 = 0;

        board.iter().for_each(|row| {
            row.iter().for_each(|piece| match piece {
                Some(GamePlayer::FirstPlayer) => first_count += 1,
                Some(GamePlayer::SecondPlayer) => second_count += 1,
                None => (),
            })
        });

        if first_count - second_count < 0 {
            return Err("The second Player has too many pieces");
        }
        if first_count - second_count > 1 {
            return Err("The first player has too many pieces");
        }

        if first_count + second_count == 42 {
            return Err("The game is already Over");
        }

        //check for floating pieces in each column
        for col in 0..BOARD_WIDTH {
            let mut seen_none = false;
            if board.iter().map(|row| row[col]).any(|p| {
                seen_none |= p.is_none();
                seen_none && p.is_some()
            }) {
                return Err("Board can not contain floating pieces");
            }
        }
        let board = GameBoard {
            board,

            total_moves: (first_count + second_count) as u8,
            winning_move: None,
        };

        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if board.max_streak(col, row) > 3 {
                    return Err("Game is already over");
                }
            }
        }

        Ok(board)
    }

    /// Test if the board when converted to a 2d array is equal the given board
    ///
    /// # Format of The Board
    /// The board is a 2d array of signed bytes. The outer array holds the array of rows.
    /// the first row is `board[0]`
    ///
    /// The pieces are represented as follows
    /// - O represents an empty spot
    /// - 1 represents a piece from the first player
    /// - -1 represents a piece from the second player
    pub fn equals_arr(&self, prev_board: &[[i8; BOARD_WIDTH]; BOARD_HEIGHT]) -> bool {
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let player = match self.board[row][col] {
                    Some(player) => player.to_int(),
                    None => 0,
                };

                if player != prev_board[row][col] {
                    return false;
                }
            }
        }

        true
    }

    /// Converts the board into an array
    ///
    /// # Format of The Board
    /// The board is a 2d array of signed bytes. The outer array holds the array of rows.
    /// the first row is `board[0]`
    ///
    /// The pieces are represented as follows
    /// - O represents an empty spot
    /// - 1 represents a piece from the first player
    /// - -1 represents a piece from the second player
    pub fn to_arr(&self) -> [[i8; BOARD_WIDTH]; BOARD_HEIGHT] {
        let mut arr = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if let Some(player) = self.board[row][col] {
                    arr[row][col] = player.to_int();
                }
            }
        }

        arr
    }
}

/// Enum which represents the different player in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlayer {
    FirstPlayer,
    SecondPlayer,
}
impl GamePlayer {
    /// Returns the other player in the game
    pub fn other(&self) -> Self {
        match self {
            Self::FirstPlayer => Self::SecondPlayer,
            Self::SecondPlayer => Self::FirstPlayer,
        }
    }

    /// Converts the player into its signed bytes representation
    /// used in a 2d array
    pub fn to_int(&self) -> i8 {
        match self {
            GamePlayer::FirstPlayer => 1,
            GamePlayer::SecondPlayer => -1,
        }
    }
}

/// Represents a valid game move. This requirement guarantees that
/// the move will be in the range of [0, BOARD_WIDTH)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameMove(u8);

impl GameMove {
    /// Returns the column associated with this move
    pub fn get_col(&self) -> u8 {
        self.0
    }

    /// Builds a GameMove with no check
    ///
    /// # Unsafe
    /// When using this you must guarantee yourself that the provided column is in the
    /// range of [0, BOARD_WIDTH). Failing to do so could cause a panic when using the move
    ///
    unsafe fn build_unchecked(col: i32) -> Self {
        GameMove(col as u8)
    }

    /// Builds a gameMove from the given column
    ///
    /// # Errors
    /// If the move is not in the range of [0, BOARD_WIDTH)
    pub fn build(col: u8) -> Result<GameMove, &'static str> {
        if col < BOARD_WIDTH as u8 {
            Ok(GameMove(col))
        } else {
            Err("Columns must be between 0 and 7")
        }
    }
}
