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
