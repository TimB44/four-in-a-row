use board::{GameBoard, GamePlayer};

mod ai_players;
pub mod board;
mod minimax;

type GameError = &'static str;

pub fn next_easy_move(board: [[i32; 7]; 6], first_player: i32) -> Result<u8, &'static str> {
    let board = parse_board(board, first_player)?;

    Ok(ai_players::play_easy(board)?.get_col())
}

pub fn next_medium_move(board: [[i32; 7]; 6], first_player: i32) -> Result<u8, &'static str> {
    let board = parse_board(board, first_player)?;

    Ok(ai_players::play_med(board)?.get_col())
}

pub fn next_hard_move(board: [[i32; 7]; 6], first_player: i32) -> Result<u8, &'static str> {
    let board = parse_board(board, first_player)?;

    Ok(ai_players::play_hard(board)?.get_col())
}

fn parse_board(num_board: [[i32; 7]; 6], first_player: i32) -> Result<GameBoard, &'static str> {
    let mut board = [[None; 7]; 6];

    for (row_index, row) in num_board.iter().enumerate() {
        for (col_index, piece) in row.iter().enumerate() {
            board[row_index][col_index] = match piece {
                1 => Some(GamePlayer::Opponent),
                0 => None,
                -1 => Some(GamePlayer::Computer),
                _ => return Err("Board can only contain 1, 0 and -1"),
            };
        }
    }
    let player = match first_player {
        1 => GamePlayer::Opponent,
        -1 => GamePlayer::Computer,
        _ => return Err("Player must be 1 or -1"),
    };

    GameBoard::build(board, player)
}
