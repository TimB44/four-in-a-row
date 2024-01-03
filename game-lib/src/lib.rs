use board::{Players, GameBoard};
use serde::{Deserialize, Serialize};

mod ai_players;
mod board;

pub fn next_easy_move_json(json: &str) -> Result<u8, &'static str> {
    

    let json_board: JsonBoard = match serde_json::from_str(json) {
        Ok(val) => val,
        Err(_) => return Err("Could not parse json")
    };

    let board = parse_board(json_board.board)?;

    Ok(ai_players::play_easy(board).get_col())
}

pub fn next_easy_move(board: [[i32; 7]; 6]) -> Result<u8, &'static str> {
    let board = parse_board(board)?;

    Ok(ai_players::play_easy(board).get_col())
}

fn parse_board(num_board: [[i32; 7]; 6]) -> Result<GameBoard, &'static str> {
    let mut board = [[None; 7]; 6];

    for (row_index, row) in num_board.iter().enumerate() {
        for (col_index, piece) in row.iter().enumerate() {
            board[row_index][col_index] = match piece {
                1 => Some(Players::Opp),
                0 => None,
                -1 => Some(Players::You),
                _ => return Err("Board can only contain 1, 0 and -1")
            };
        }
    }

    GameBoard::build(board)
}

#[derive(Serialize, Deserialize)]
struct JsonBoard {
    pub board: [[i32; 7]; 6],
}

#[cfg(test)]
mod parse_world_json_test {
    // use super::*;
    // #[test]
    // fn json_test_simple() {
    //     let json = r#"
    //     {
    //         "board": [
    //             [0, 0, 0, 0, 0, 0, 0],
    //             [0, 0, 0, 0, 0, 0, 0],
    //             [0, 0, 0, 0, 0, 0, 0],
    //             [0, 0, 0, 0, 0, 0, 0],
    //             [0, 0, 0, 0, 0, 0, 0],
    //             [0, 0, 0, 0, 0, 0, 0]
    //         ]
    //     }"#;
    
    //     let game_board = parse_board_json(json).unwrap();

    //     for row in game_board.get_board() {
    //         for piece in row {
    //             assert!(piece.is_none());
    //         }
    //     }
    // }
}
