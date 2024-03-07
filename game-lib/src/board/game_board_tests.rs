use crate::board::*;

#[test]
fn cause_win_vertical_left() {
    let mut board = [[None; 7]; 6];
    board[0][0] = Some(GamePlayer::Opponent);
    board[1][0] = Some(GamePlayer::Opponent);
    board[2][0] = Some(GamePlayer::Opponent);

    board[0][1] = Some(GamePlayer::Computer);
    board[1][1] = Some(GamePlayer::Computer);
    board[2][1] = Some(GamePlayer::Computer);

    let mut gb = GameBoard::build(board, GamePlayer::Opponent).unwrap();

    gb.make_move(&GameMove::build(0).unwrap()).unwrap();

    assert!(gb.is_over() && gb.winner() == Some(GamePlayer::Opponent));
}

#[test]
fn cause_win_vertical_mid() {
    let mut board = [[None; 7]; 6];
    board[0][3] = Some(GamePlayer::Computer);
    board[1][3] = Some(GamePlayer::Computer);
    board[2][3] = Some(GamePlayer::Computer);

    board[0][0] = Some(GamePlayer::Opponent);
    board[1][0] = Some(GamePlayer::Opponent);
    board[2][0] = Some(GamePlayer::Opponent);

    let mut gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

    gb.make_move(&GameMove::build(3).unwrap()).unwrap();
    assert!(
        gb.is_over()
            && gb.winner() == Some(GamePlayer::Computer)
            && gb.winning_move == Some(GameMove(3))
    );
}

#[test]
fn cause_win_vertical_right() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(GamePlayer::Computer);
    board[1][6] = Some(GamePlayer::Computer);
    board[2][6] = Some(GamePlayer::Computer);

    board[0][0] = Some(GamePlayer::Opponent);
    board[1][0] = Some(GamePlayer::Opponent);
    board[2][0] = Some(GamePlayer::Opponent);

    let mut gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

    gb.make_move(&GameMove::build(6).unwrap()).unwrap();
    assert!(
        gb.is_over()
            && gb.winner() == Some(GamePlayer::Computer)
            && gb.winning_move == Some(GameMove(6))
    );
}

#[test]
fn wont_cause_win_not_same_player() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(GamePlayer::Computer);
    board[1][6] = Some(GamePlayer::Opponent);
    board[2][6] = Some(GamePlayer::Computer);

    let mut gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

    gb.make_move(&GameMove::build(6).unwrap()).unwrap();
    assert!(!gb.is_over() && gb.winner() == None && gb.winning_move == None);
}

#[test]
fn wont_cause_win_3_streak() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(GamePlayer::Computer);
    board[1][6] = Some(GamePlayer::Computer);

    board[0][0] = Some(GamePlayer::Opponent);
    board[1][0] = Some(GamePlayer::Opponent);


    let mut gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

    gb.make_move(&GameMove::build(6).unwrap()).unwrap();
    assert!(!gb.is_over() && gb.winner() == None && gb.winning_move == None);
    assert_eq!(gb.max_streak(6, 2), 3);
}