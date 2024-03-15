use crate::{
    board::{GameBoard, GameMove},
    minimax::minimax,
    GameError,
};

/// Finds a move by performing a minimax game tree search with a depth of 8
pub fn play_easy(board: GameBoard) -> Result<GameMove, GameError> {
    minimax(board, 2)
}

/// Finds a move by performing a minimax game tree search with a depth of 8
pub fn play_med(board: GameBoard) -> Result<GameMove, GameError> {
    minimax(board, 4)
}

/// Finds a move by performing a minimax game tree search with a depth of 8
pub fn play_hard(board: GameBoard) -> Result<GameMove, GameError> {
    minimax(board, 10)
}

#[cfg(test)]
mod tests {
    use crate::{
        ai_players::*,
        board::{GameBoard, GameMove, GamePlayer},
    };

    #[test]
    fn play_easy_block() {
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::FirstPlayer);
        board[1][0] = Some(GamePlayer::FirstPlayer);
        board[2][0] = Some(GamePlayer::FirstPlayer);

        board[0][4] = Some(GamePlayer::SecondPlayer);
        board[1][4] = Some(GamePlayer::SecondPlayer);
        board[0][5] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board).unwrap();

        assert_eq!(play_easy(gb), GameMove::build(0));
    }

    #[test]
    fn play_easy_win() {
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::SecondPlayer);
        board[1][0] = Some(GamePlayer::SecondPlayer);
        board[2][0] = Some(GamePlayer::SecondPlayer);

        board[0][4] = Some(GamePlayer::FirstPlayer);
        board[1][4] = Some(GamePlayer::FirstPlayer);
        board[2][4] = Some(GamePlayer::FirstPlayer);

        let gb = GameBoard::build(board).unwrap();

        assert_eq!(play_easy(gb), GameMove::build(4));
    }

    #[test]
    fn play_easy_win_second() {
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::FirstPlayer);
        board[1][0] = Some(GamePlayer::FirstPlayer);
        board[2][0] = Some(GamePlayer::FirstPlayer);
        board[0][1] = Some(GamePlayer::FirstPlayer);

        board[0][4] = Some(GamePlayer::SecondPlayer);
        board[1][4] = Some(GamePlayer::SecondPlayer);
        board[2][4] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board).unwrap();

        assert_eq!(play_easy(gb), GameMove::build(4));
    }

    #[test]
    fn play_med_block() {
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::SecondPlayer);
        board[1][0] = Some(GamePlayer::SecondPlayer);
        board[2][0] = Some(GamePlayer::SecondPlayer);

        board[0][4] = Some(GamePlayer::FirstPlayer);
        board[1][4] = Some(GamePlayer::FirstPlayer);
        board[0][5] = Some(GamePlayer::FirstPlayer);

        let gb = GameBoard::build(board).unwrap();

        assert_eq!(play_med(gb), GameMove::build(0));
    }

    #[test]
    fn play_med_win() {
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::SecondPlayer);
        board[1][0] = Some(GamePlayer::SecondPlayer);
        board[2][0] = Some(GamePlayer::SecondPlayer);

        board[0][4] = Some(GamePlayer::FirstPlayer);
        board[1][4] = Some(GamePlayer::FirstPlayer);
        board[2][4] = Some(GamePlayer::FirstPlayer);

        let gb = GameBoard::build(board).unwrap();

        assert_eq!(play_med(gb), GameMove::build(4));
    }

    #[test]
    fn play_hard_block() {
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::SecondPlayer);
        board[1][0] = Some(GamePlayer::SecondPlayer);
        board[2][0] = Some(GamePlayer::SecondPlayer);

        board[0][4] = Some(GamePlayer::FirstPlayer);
        board[1][4] = Some(GamePlayer::FirstPlayer);
        board[0][5] = Some(GamePlayer::FirstPlayer);

        let gb = GameBoard::build(board).unwrap();

        assert_eq!(play_hard(gb), GameMove::build(0));
    }

    #[test]
    fn play_hard_win() {
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::SecondPlayer);
        board[1][0] = Some(GamePlayer::SecondPlayer);
        board[2][0] = Some(GamePlayer::SecondPlayer);

        board[0][4] = Some(GamePlayer::FirstPlayer);
        board[1][4] = Some(GamePlayer::FirstPlayer);
        board[2][4] = Some(GamePlayer::FirstPlayer);

        let gb = GameBoard::build(board).unwrap();

        assert_eq!(play_hard(gb), GameMove::build(4));
    }

    #[test]
    fn play_hard_finishes() {
        let mut board = [[None; 7]; 6];
        board[0][3] = Some(GamePlayer::SecondPlayer);
        board[0][4] = Some(GamePlayer::FirstPlayer);

        let gb = GameBoard::build(board).unwrap();
        println!("{:?}", play_hard(gb).unwrap());
    }

    #[test]
    fn play_hard_mid_game() {
        let mut board = [[None; 7]; 6];

        board[0][2] = Some(GamePlayer::FirstPlayer);
        board[0][3] = Some(GamePlayer::FirstPlayer);

        board[1][3] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board).unwrap();
        let gm = play_med(gb).unwrap().get_col();
        assert!(gm == 1 || gm == 4);
    }
}
