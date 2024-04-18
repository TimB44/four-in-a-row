use crate::board::*;

#[test]
fn cause_win_vertical_left() {
    let mut board = [[None; 7]; 6];
    board[0][0] = Some(GamePlayer::FirstPlayer);
    board[1][0] = Some(GamePlayer::FirstPlayer);
    board[2][0] = Some(GamePlayer::FirstPlayer);

    board[0][1] = Some(GamePlayer::SecondPlayer);
    board[1][1] = Some(GamePlayer::SecondPlayer);
    board[2][1] = Some(GamePlayer::SecondPlayer);

    let mut gb = GameBoard::build(board).unwrap();

    gb.make_move(&GameMove::build(0).unwrap()).unwrap();

    assert!(gb.is_over());
    assert_eq!(gb.winner(), Some(GamePlayer::FirstPlayer));
}

#[test]
fn cause_win_vertical_mid() {
    let mut board = [[None; 7]; 6];
    board[0][3] = Some(GamePlayer::FirstPlayer);
    board[1][3] = Some(GamePlayer::FirstPlayer);
    board[2][3] = Some(GamePlayer::FirstPlayer);

    board[0][0] = Some(GamePlayer::SecondPlayer);
    board[1][0] = Some(GamePlayer::SecondPlayer);
    board[2][0] = Some(GamePlayer::SecondPlayer);

    let mut gb = GameBoard::build(board).unwrap();

    gb.make_move(&GameMove::build(3).unwrap()).unwrap();
    assert!(gb.is_over());
    assert_eq!(gb.winner(), Some(GamePlayer::FirstPlayer));
    assert_eq!(gb.winning_move, Some(GameMove(3)));
}

#[test]
fn cause_win_vertical_right() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(GamePlayer::FirstPlayer);
    board[1][6] = Some(GamePlayer::FirstPlayer);
    board[2][6] = Some(GamePlayer::FirstPlayer);

    board[0][0] = Some(GamePlayer::SecondPlayer);
    board[1][0] = Some(GamePlayer::SecondPlayer);
    board[2][0] = Some(GamePlayer::SecondPlayer);

    let mut gb = GameBoard::build(board).unwrap();

    gb.make_move(&GameMove::build(6).unwrap()).unwrap();
    assert!(gb.is_over());
    assert_eq!(gb.winner(), Some(GamePlayer::FirstPlayer));
    assert_eq!(gb.winning_move, Some(GameMove(6)));
}

#[test]
fn wont_cause_win_not_same_player() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(GamePlayer::FirstPlayer);
    board[2][6] = Some(GamePlayer::FirstPlayer);

    board[1][6] = Some(GamePlayer::SecondPlayer);

    let mut gb = GameBoard::build(board).unwrap();

    gb.make_move(&GameMove::build(6).unwrap()).unwrap();
    assert!(!gb.is_over());
    assert_eq!(gb.winner(), None);
    assert!(gb.winning_move.is_none());
}

#[test]
fn wont_cause_win_3_streak() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(GamePlayer::FirstPlayer);
    board[1][6] = Some(GamePlayer::FirstPlayer);

    board[0][0] = Some(GamePlayer::SecondPlayer);
    board[1][0] = Some(GamePlayer::SecondPlayer);

    let mut gb = GameBoard::build(board).unwrap();

    gb.make_move(&GameMove::build(6).unwrap()).unwrap();
    assert!(!gb.is_over());
    assert!(gb.winner().is_none());
    assert!(gb.winning_move.is_none());
    assert_eq!(gb.max_streak(6, 2), 3);
}

// Build Tests
#[test]
fn build_floating_piece() {
    for i in 0..7 {
        let j = (i + 1) % 7;
        let mut board = [[None; 7]; 6];
        board[0][j] = Some(GamePlayer::FirstPlayer);
        board[1][j] = Some(GamePlayer::FirstPlayer);

        board[1][i] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board);
        assert!(gb.is_err());
    }

    for i in 0..7 {
        let j = (i + 1) % 7;
        let mut board = [[None; 7]; 6];
        board[1][j] = Some(GamePlayer::FirstPlayer);
        board[2][j] = Some(GamePlayer::FirstPlayer);

        board[0][i] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board);
        assert!(gb.is_err());
    }
}

#[test]
fn build_invalid_piece_count() {
    for i in 0..7 {
        let mut board = [[None; 7]; 6];
        board[0][i] = Some(GamePlayer::FirstPlayer);
        board[1][i] = Some(GamePlayer::FirstPlayer);

        let mut j = (2 * i) % 7;
        if i == j {
            j = (j + 1) % 7;
        }

        board[0][j] = Some(GamePlayer::SecondPlayer);
        board[1][j] = Some(GamePlayer::SecondPlayer);
        board[2][j] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board);
        assert!(gb.is_err());
    }

    for i in 0..7 {
        let mut board = [[None; 7]; 6];
        board[0][i] = Some(GamePlayer::FirstPlayer);
        board[1][i] = Some(GamePlayer::FirstPlayer);
        board[2][i] = Some(GamePlayer::FirstPlayer);

        let mut j = (2 * i) % 7;
        if i == j {
            j = (j + 1) % 7;
        }

        board[0][j] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board);
        assert!(gb.is_err());
    }
}

// Max Streak tests

#[test]
fn streak_zero() {
    for i in 0..7 {
        let mut board = [[None; 7]; 6];
        board[0][i] = Some(GamePlayer::FirstPlayer);
        board[1][i] = Some(GamePlayer::FirstPlayer);

        let mut j = (2 + i) % 7;
        if i == j {
            j = (j + 1) % 7;
        }

        board[0][j] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board).unwrap();
        assert_eq!(gb.max_streak((i + 4) % 7, 0), 0);
    }
}
#[test]
fn streak_one_and_two() {
    for i in 0..7 {
        let mut board = [[None; 7]; 6];
        board[0][i] = Some(GamePlayer::FirstPlayer);
        board[1][i] = Some(GamePlayer::FirstPlayer);

        let mut j = (2 + i) % 7;
        if i == j {
            j = (j + 1) % 7;
        }

        board[0][j] = Some(GamePlayer::SecondPlayer);

        let gb = GameBoard::build(board).unwrap();

        assert_eq!(gb.max_streak(i, 0), 2);
        assert_eq!(gb.max_streak(i, 1), 2);

        assert_eq!(gb.max_streak(j, 0), 1);

        assert_eq!(gb.max_streak(i, 2), 0);
        assert_eq!(gb.max_streak(j, 1), 0);
    }
}

#[test]
fn left_diagonal_down_streak() {
    let mut board = [[None; 7]; 6];
    board[0][0] = Some(GamePlayer::FirstPlayer);
    board[1][1] = Some(GamePlayer::FirstPlayer);

    board[0][1] = Some(GamePlayer::SecondPlayer);

    let gb = GameBoard::build(board).unwrap();

    assert_eq!(gb.max_streak(1, 1), 2);
    assert_eq!(gb.max_streak(0, 0), 2);

    let mut board = [[None; 7]; 6];
    board[0][0] = Some(GamePlayer::SecondPlayer);
    board[1][1] = Some(GamePlayer::SecondPlayer);

    board[0][1] = Some(GamePlayer::FirstPlayer);
    board[0][2] = Some(GamePlayer::FirstPlayer);

    let gb = GameBoard::build(board).unwrap();

    assert_eq!(gb.max_streak(1, 1), 2);
    assert_eq!(gb.max_streak(0, 0), 2);
}

#[test]
fn right_diagonal_down_streak() {
    let mut board = [[None; 7]; 6];
    board[0][1] = Some(GamePlayer::FirstPlayer);
    board[1][0] = Some(GamePlayer::FirstPlayer);

    board[0][0] = Some(GamePlayer::SecondPlayer);

    let gb = GameBoard::build(board).unwrap();

    assert_eq!(gb.max_streak(0, 1), 2);
    assert_eq!(gb.max_streak(1, 0), 2);
    

    let mut board = [[None; 7]; 6];
    board[1][0] = Some(GamePlayer::SecondPlayer);
    board[0][1] = Some(GamePlayer::SecondPlayer);

    board[0][0] = Some(GamePlayer::FirstPlayer);
    board[0][2] = Some(GamePlayer::FirstPlayer);

    let gb = GameBoard::build(board).unwrap();

    assert_eq!(gb.max_streak(1, 0), 2);
    assert_eq!(gb.max_streak(0, 1), 2);
}

#[test]
fn horizontal_streak() {
    let mut board = [[None; 7]; 6];
    board[0][1] = Some(GamePlayer::FirstPlayer);
    board[1][1] = Some(GamePlayer::FirstPlayer);
    board[1][2] = Some(GamePlayer::FirstPlayer);
    board[1][3] = Some(GamePlayer::FirstPlayer);

    board[0][2] = Some(GamePlayer::SecondPlayer);
    board[0][3] = Some(GamePlayer::SecondPlayer);
    board[0][4] = Some(GamePlayer::SecondPlayer);

    let gb = GameBoard::build(board).unwrap();

    assert_eq!(gb.max_streak(1, 0), 2);
    for i in 1..=3 {
        assert_eq!(gb.max_streak(i, 1), 3);
    }

    for i in 2..=4 {
        assert_eq!(gb.max_streak(i, 0), 3);
    }
}

#[test]
fn horizontal_streak_win() {
    let mut board = [[None; 7]; 6];
    board[0][1] = Some(GamePlayer::FirstPlayer);
    board[1][1] = Some(GamePlayer::FirstPlayer);
    board[1][2] = Some(GamePlayer::FirstPlayer);
    board[1][3] = Some(GamePlayer::FirstPlayer);

    board[0][2] = Some(GamePlayer::SecondPlayer);
    board[0][3] = Some(GamePlayer::SecondPlayer);
    board[0][4] = Some(GamePlayer::SecondPlayer);


    let mut gb = GameBoard::build(board).unwrap();

    assert!(!gb.is_over());
    gb.make_move(&GameMove::build(5).unwrap()).unwrap();
    assert!(gb.is_over());
    for i in 2..=5 {
        assert_eq!(gb.max_streak(i, 0), 4);
    }
}