use crate::GameError;
use std::cmp::{max, min};
#[cfg(test)]
mod game_board_tests;

#[derive(Debug, Clone, Copy)]
pub struct GameBoard {
    board: [[Option<GamePlayer>; 7]; 6], // board[0] is the bottom row
    total_moves: u8,
    winning_move: Option<GameMove>,
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            board: [[None; 7]; 6],
            total_moves: 0,
            winning_move: None,
        }
    }
    pub fn make_move(&mut self, game_move: &GameMove) -> Result<(), GameError> {
        let col = game_move.0 as usize;
        if self.is_over() {
            return Err("Can not make move when game is over");
        }

        let player = self.current_player();
        for i in 0..7 {
            if self.board[i][col as usize].is_none() {
                self.board[i][col as usize] = Some(player);
                if self.max_streak(col, i) > 3 {
                    self.winning_move = Some(*game_move)
                }
                self.total_moves += 1;
                return Ok(());
            }
        }

        Err("Can not make move because given column is full")
    }

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

        for i in (0..6).rev() {
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

    pub fn current_player(&self) -> GamePlayer {
        if self.total_moves % 2 == 0 {
            GamePlayer::FirstPlayer
        } else {
            GamePlayer::SecondPlayer
        }
    }

    pub fn is_over(&self) -> bool {
        self.winning_move.is_some() || self.total_moves == 42
    }

    pub fn winner(&self) -> Option<GamePlayer> {
        if !self.is_over() || self.total_moves == 42 {
            return None;
        }

        Some(self.current_player().other())
    }

    pub fn moves<'a>(&'a self) -> Vec<GameMove> {
        self.board[5]
            .iter()
            .enumerate()
            .filter(|(_, spot)| !self.is_over() && spot.is_none())
            .map(|(col, _)| unsafe { GameMove::build_unchecked(col as i32) })
            .collect()
    }

    fn max_streak(&self, col: usize, row: usize) -> u8 {
        let player = match self.board[row][col] {
            None => return 0,
            Some(player) => player,
        };

        let mut streak = 1;
        for i in (0..row).rev() {
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
        for i in (col + 1)..7 {
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
        let max_offset = min(5 - row, 6 - col);
        for i in 1..=max_offset {
            if self.board[row + i][col + i] != Some(player) {
                break;
            }
            new_streak += 1;
        }

        streak = max(streak, new_streak);

        //top left to bottom right diagonal
        new_streak = 0;

        //left branch
        let max_offset = min(5 - row, col);
        for i in 1..=max_offset {
            if self.board[row + i][col - i] != Some(player) {
                break;
            }
            new_streak += 1;
        }

        // right branch
        let max_offset = min(row, 6 - col);
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

        for col in 0..7 {
            let mut row = 0;
            while row < 6 && self.board[row][col].is_some() {
                row += 1;
            }
            if row == 6 || row == 0 {
                continue;
            }

            let modifier = match self.board[row - 1][col].expect("Error in current score logic") {
                GamePlayer::FirstPlayer =>  1,
                GamePlayer::SecondPlayer => -1,
            };

            total += 10_i32.pow((self.max_streak(col, row - 1)).into()) * modifier;
        }
        println!("{:?}", total);
        total
        
    }

    pub fn build(board: [[Option<GamePlayer>; 7]; 6]) -> Result<GameBoard, GameError> {
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
        for col in 0..7 {
            let mut seen_none = false;
            if board
                .iter()
                .map(|row| row[col])
                .filter(|p| {
                    seen_none = seen_none | p.is_none();
                    seen_none && p.is_some()
                })
                .next()
                .is_some()
            {
                return Err("Board can not contain floating pieces");
            }
        }
        let board = GameBoard {
            board,

            total_moves: (first_count + second_count) as u8,
            winning_move: None,
        };

        for row in 0..6 {
            for col in 0..7 {
                if board.max_streak(col, row) > 3 {
                    return Err("Game is already over");
                }
            }
        }

        Ok(board)
    }

    pub fn equals_arr(&self, prev_board: &[[i8; 7]; 6]) -> bool {
        for row in 0..6 {
            for col in 0..7 {
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

    pub fn to_arr(&self) -> [[i8; 7]; 6] {
        let mut arr = [[0; 7]; 6];
        for row in 0..6 {
            for col in 0..7 {
                if let Some(player) = self.board[row][col] {
                    arr[row][col] = player.to_int();
                }
            }
        }

        arr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlayer {
    FirstPlayer,
    SecondPlayer,
}
impl GamePlayer {
    pub fn other(&self) -> Self {
        match self {
            Self::FirstPlayer => Self::SecondPlayer,
            Self::SecondPlayer => Self::FirstPlayer,
        }
    }

    pub fn to_int(&self) -> i8 {
        match self {
            GamePlayer::FirstPlayer => 1,
            GamePlayer::SecondPlayer => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameMove(u8);

impl GameMove {
    pub fn get_col(&self) -> u8 {
        self.0
    }

    unsafe fn build_unchecked(col: i32) -> Self {
        GameMove(col as u8)
    }

    pub fn build(col: u8) -> Result<GameMove, &'static str> {
        if col < 7 {
            Ok(GameMove(col))
        } else {
            Err("Columns must be between 0 and 7")
        }
    }
}
