use crate::board::{GameBoard, GameMove, Players};

/// Simple algorithm which Searches for possible wins, or blocks possible losses.
/// If neither occur then it will place semi-randomly
pub fn play_easy(board: GameBoard) -> GameMove {
    for col in 0..=7 {
        let game_move = GameMove::build(col).unwrap();

        if will_cause_win(&board, &Players::You, game_move) {
            return game_move;
        }
    }

    for col in 0..=7 {
        let game_move = GameMove::build(col).unwrap();

        if will_cause_win(&board, &Players::Opp, game_move) {
            return game_move;
        }
    }

    todo!();
}

fn will_cause_win(game_board: &GameBoard, player: &Players, game_move: GameMove) -> bool {
    let col: usize = game_move.get_col().into();
    let board = game_board.board;

    // top is the spot will the piece will land. 
    let top = board
        .iter()
        .map(|row| row[col])
        .take_while(|p| p.is_some())
        .count();

    //if top is greater then six, the move is invalid, and the can not cause a win. 
    if top > 5 {
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
    //todo

    //top left to bottom right diagonal win
    //todo

    
    false
}

#[cfg(test)]
mod tests {
    use crate::{ai_players::will_cause_win, board::*};

    #[test]
    fn cause_win_vertical_left() {
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(Players::Opp);
        board[1][0] = Some(Players::Opp);
        board[2][0] = Some(Players::Opp);

        let gb = GameBoard::build(board).unwrap();

        assert!(will_cause_win(
            &gb,
            &Players::Opp,
            GameMove::build(0).unwrap()
        ))
    }

    #[test]
    fn cause_win_vertical_mid() {
        let mut board = [[None; 7]; 6];
        board[0][3] = Some(Players::You);
        board[1][3] = Some(Players::You);
        board[2][3] = Some(Players::You);

        let gb = GameBoard::build(board).unwrap();

        assert!(will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(3).unwrap()
        ))
    }

    #[test]
    fn cause_win_vertical_right() {
        let mut board = [[None; 7]; 6];
        board[0][6] = Some(Players::You);
        board[1][6] = Some(Players::You);
        board[2][6] = Some(Players::You);

        let gb = GameBoard::build(board).unwrap();

        assert!(will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(6).unwrap()
        ))
    }

    #[test]
    fn wont_cause_win_not_same_player() {
        let mut board = [[None; 7]; 6];
        board[0][6] = Some(Players::You);
        board[1][6] = Some(Players::Opp);
        board[2][6] = Some(Players::You);

        let gb = GameBoard::build(board).unwrap();

        assert!(!will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(6).unwrap()
        ))
    }

    #[test]
    fn wont_cause_win_3_streak() {
        let mut board = [[None; 7]; 6];
        board[0][6] = Some(Players::You);
        board[1][6] = Some(Players::You);

        let gb = GameBoard::build(board).unwrap();

        assert!(!will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(6).unwrap()
        ))
    }

    #[test]
    fn cause_win_horizontal() {
        let mut board = [[None; 7]; 6];
        board[0][4] = Some(Players::You);
        board[0][5] = Some(Players::You);
        board[0][6] = Some(Players::You);

        let gb = GameBoard::build(board).unwrap();

        assert!(will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(3).unwrap()
        ));

        board[0][3] = Some(Players::Opp);
        board[1][4] = Some(Players::You);
        board[1][5] = Some(Players::You);
        board[1][6] = Some(Players::You);

        assert!(will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(3).unwrap()
        ));
    }

    #[test]
    fn cause_win_horizontal_split() {
        let mut board = [[None; 7]; 6];
        board[0][3] = Some(Players::You);
        board[0][5] = Some(Players::You);
        board[0][6] = Some(Players::You);

        let gb = GameBoard::build(board).unwrap();

        assert!(will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(4).unwrap()
        ));
    }

    #[test]
    fn cause_win_horizontal_edge() {
        let mut board = [[None; 7]; 6];

        board[0][3] = Some(Players::You);
        board[0][4] = Some(Players::Opp);
        board[0][5] = Some(Players::You);
        board[0][6] = Some(Players::You);

        board[1][3] = Some(Players::You);
        board[1][4] = Some(Players::You);
        board[1][5] = Some(Players::You);

        let gb = GameBoard::build(board).unwrap();

        assert!(will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(6).unwrap()
        ));
    }

    #[test]
    fn wont_cause_win_bad_horizontal_split() {
        let mut board = [[None; 7]; 6];

        board[0][3] = Some(Players::You);

        board[0][5] = Some(Players::Opp);
        board[0][6] = Some(Players::You);

        board[1][3] = Some(Players::You);
        board[1][5] = Some(Players::You);
        board[1][6] = Some(Players::You);

        let gb = GameBoard::build(board).unwrap();

        assert!(!will_cause_win(
            &gb,
            &Players::You,
            GameMove::build(4).unwrap()
        ));
    }
}
