use crate::GameError;
use std::cmp::{max, min};
#[cfg(test)]
mod game_board_tests;

#[derive(Debug, Clone, Copy)]
pub struct GameBoard {
    board: [[Option<GamePlayer>; 7]; 6], // board[0] is the bottom row
    first_player: GamePlayer,
    total_moves: u8,
    winning_move: Option<GameMove>,
}

impl GameBoard {
    pub fn make_move(&mut self, game_move: &GameMove) -> Result<(), GameError> {
        let col = game_move.0 as usize;
        if self.is_over() {
            return Err("Can not make move when game is over");
        }

        let player = self.current_player();
        for i in 0..7 {
            if self.board[i][col as usize].is_none() {
                self.board[i][col as usize] = Some(player);
                if self.max_streak(col, i) > 3{
                    self.winning_move = Some(*game_move)
                }
                self.total_moves += 1;
                return Ok(());
            }
        }

        Err("Can not make move as column is full")
    }

    pub fn undo_move(&mut self, game_move: &GameMove) -> Result<(), GameError> {
        let col = game_move.0;
        if self.is_over() {
            if col != self.winning_move.unwrap().0 {
                return Err("If the game is over can only remove the piece last played");
            }
            self.winning_move = None;
        }

        for i in (0..7).rev() {
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
            self.first_player
        } else {
            self.first_player.other()
        }
    }

    pub fn is_over(&self) -> bool {
        self.winning_move.is_some()
    }

    pub fn winner(&self) -> Option<GamePlayer> {
        if self.is_over() {
            return None;
        }

        Some(self.current_player().other())
    }

    pub fn moves<'a>(&'a self) -> impl Iterator<Item = GameMove> + 'a {
        self.board[5]
            .iter()
            .enumerate()
            .filter(|(_, spot)| !self.is_over() && spot.is_none())
            .map(|(col, _)| unsafe { GameMove::build_unchecked(col as i32) })
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

        return max(streak, new_streak);
    }

    pub fn current_score(&self) -> i32 {
        todo!()
    }


    pub fn build(
        board: [[Option<GamePlayer>; 7]; 6],
        first_player: GamePlayer,
    ) -> Result<GameBoard, GameError> {
        //check for invalid number of pieces
        let mut first_count: i32 = 0;
        let mut second_count: i32 = 0;
    
        board.iter().for_each(|row| {
            row.iter().for_each(|piece| match piece {
                Some(player) => match player {
                    first_player => first_count += 1,
                    _ => second_count += 1,
                },
                _ => (),
            })
        });
    
        if first_count - second_count < 0  {
            return Err("The Second Player has too many pieces");
        }
        if first_count - second_count > 1 {
            return Err("The first player has too many pieces");
        }
    
    
        if first_count + second_count == 42 {
            return Err("The Game is already Over");
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
                .next().is_some()
            {
                return Err("Board can not contain floating pieces");
            }
        }
        let board = GameBoard {
            board, 
            first_player, 
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
    
        return Ok(board);
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlayer {
    Opponent,
    Computer,
}
impl GamePlayer {
    pub fn other(&self) -> Self {
        match self {
            Self::Opponent => Self::Computer,
            Self::Computer => Self::Opponent,
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
}

impl GameMove {
    pub fn build(col: u8) -> Result<GameMove, &'static str> {
        if col < 7 {
            Ok(GameMove(col))
        } else {
            Err("Columns must be between 0 and 7")
        }
    }
}
