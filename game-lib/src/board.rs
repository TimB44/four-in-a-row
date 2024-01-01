#[cfg(test)]
mod game_board_tests;

#[derive(Debug, Clone, Copy)]
pub struct GameBoard {
    board: [[Option<Players>; 7]; 6], // board[0] is the bottom row
    _private: (),
}

impl GameBoard {
    pub fn get_board(&self) -> &[[Option<Players>; 7]; 6] {
        &self.board
    }

    ///Builds a game board that is not yet over, which will be valid.
    pub fn build(board: [[Option<Players>; 7]; 6]) -> Result<GameBoard, &'static str> {
        //todo check for valid states

        //check for invalid number of pieces
        let mut opp_count: i32 = 0;
        let mut you_count: i32 = 0;
        for row in board {
            for piece in row {
                match piece {
                    Some(player) => match player {
                        Players::You => you_count += 1,
                        Players::Opp => opp_count += 1,
                    },
                    None => continue,
                }
            }
        }

        if (opp_count - you_count).abs() > 1 {
            if you_count > opp_count {
                return Err("The AI has too many pieces");
            } else {
                return Err("The Player has to many pieces");
            }
        }

        if opp_count + you_count == 42 {
            return Err("The Game is already Over");
        }

        //check for floating pieces in each column
        for col in 0..7 {
            let mut seen_none = false;
            if board
                .iter()
                .map(|row| row[col])
                .filter(|p| {
                    seen_none = p.is_none();
                    seen_none
                })
                .any(|p| p.is_some())
            {
                return Err("Board can not contain floating pieces");
            }
        }

        //todo check for wins maybe?

        Ok(GameBoard {
            board,
            _private: (),
        })
    }

    pub fn will_cause_win(&self, player: &Players, game_move: GameMove) -> bool {
        let col: usize = game_move.get_col().into();
        let board = self.board;

        // top is the spot will the piece will land.
        let top = board
            .iter()
            .map(|row| row[col])
            .take_while(|p| p.is_some())
            .count();

        //if top is greater then six, the move is invalid, and the can not cause a win.
        if top > 6 {
            return false;
        }

        //vertical win
        let streak = board
            .iter()
            .rev()
            .map(|row| row[col])
            .filter(|piece| piece.is_some())
            .take_while(|p| p.unwrap() == *player)
            .count();

        if streak >= 3 {
            return true;
        }

        //horizontal win
        let mut streak = 0;
        let mut temp_streak = 0;
        board[top]
            .into_iter()
            .enumerate()
            .take_while(|(i, _)| *i < col)
            .for_each(|(_, p)| {
                if p.is_some() && p.unwrap() == *player {
                    temp_streak += 1;
                } else {
                    temp_streak = 0;
                }
            });

        streak += temp_streak;
        temp_streak = 0;
        board[top]
            .into_iter()
            .enumerate()
            .rev()
            .take_while(|(i, _)| *i > col)
            .for_each(|(_, p)| {
                if p.is_some() && p.unwrap() == *player {
                    temp_streak += 1;
                } else {
                    temp_streak = 0;
                }
            });

        streak += temp_streak;

        if streak >= 3 {
            return true;
        }

        //bottom left to top right diagonal win

        let mut streak = 0;
        let mut cur_col = col + 1;
        let mut cur_row = top + 1;
        while cur_col < 7 && cur_row < 6 {
            match board[cur_row][cur_col] {
                Some(p) if &p == player => streak += 1,
                _ => break,
            }
            cur_col += 1;
            cur_row += 1
        }

        let mut cur_col = col.checked_sub(1).unwrap_or(usize::MAX);
        let mut cur_row = top.checked_sub(1).unwrap_or(usize::MAX);
        while cur_col != usize::MAX && cur_row != usize::MAX {
            match board[cur_row][cur_col] {
                Some(p) if &p == player => streak += 1,
                _ => break,
            }

            cur_col = cur_col.checked_sub(1).unwrap_or(usize::MAX);
            cur_row = cur_row.checked_sub(1).unwrap_or(usize::MAX);
        }

        if streak >= 3 {
            return true;
        }

        //top left to bottom right diagonal win
        let mut streak = 0;
        let mut cur_col = col.checked_sub(1).unwrap_or(usize::MAX);
        let mut cur_row = top + 1;
        while cur_col != usize::MAX && cur_row < 6 {
            match board[cur_row][cur_col] {
                Some(p) if &p == player => streak += 1,
                _ => break,
            }
            cur_col = cur_col.checked_sub(1).unwrap_or(usize::MAX);
            cur_row += 1
        }

        let mut cur_col = col + 1;
        let mut cur_row = top.checked_sub(1).unwrap_or(usize::MAX);
        while cur_col < 7 && cur_row != usize::MAX {
            match board[cur_row][cur_col] {
                Some(p) if &p == player => streak += 1,
                _ => break,
            }

            cur_col += 1;
            cur_row = cur_row.checked_sub(1).unwrap_or(usize::MAX);
        }

        if streak >= 3 {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Players {
    Opp,
    You,
}

#[derive(Debug, Clone, Copy)]
pub struct GameMove(u8);

impl GameMove {
    pub fn get_col(&self) -> u8 {
        self.0
    }
}

impl GameMove {
    pub fn build(col: u8) -> Result<GameMove, &'static str> {
        if col < 8 {
            Ok(GameMove(col))
        } else {
            Err("Columns must be between 0 and 7")
        }
    }
}
