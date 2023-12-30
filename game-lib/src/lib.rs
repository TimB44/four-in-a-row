mod ai_players;
mod board;

pub fn next_easy_move(json: &str) -> Result<u8, &'static str> {
    let board = parse_board_json(json)?;

    Ok(ai_players::play_easy(board).get_col())
}

fn parse_board_json(_json: &str) -> Result<board::GameBoard, &'static str> {
    board::GameBoard::build([[None; 7]; 6])

    //todo parse json data
}
