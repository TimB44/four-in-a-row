#[derive(Debug, Clone, Copy)]
pub struct GameBoard {
    pub board: [[Option<Players>; 7]; 6], // board[0] is the bottom row
    _private: (),
}

impl GameBoard {
    pub fn build(board: [[Option<Players>; 7]; 6]) -> Result<GameBoard, &'static str> {
        //todo check for valid states

        Ok(GameBoard {
            board,
            _private: (),
        })
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
