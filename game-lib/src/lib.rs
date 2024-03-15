use board::{GameBoard, GamePlayer};

mod ai_players;
pub mod board;
mod minimax;

type GameError = &'static str;

pub fn next_easy_move(board: [[i8; 7]; 6]) -> Result<u8, GameError> {
    let board = parse_board(board)?;

    Ok(ai_players::play_easy(board)?.get_col())
}

pub fn next_medium_move(board: [[i8; 7]; 6]) -> Result<u8, GameError> {
    let board = parse_board(board)?;

    Ok(ai_players::play_med(board)?.get_col())
}

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
