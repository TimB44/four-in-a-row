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

    let mut gb = GameBoard::build(board, GamePlayer::Opponent).unwrap();

    gb.make_move(&GameMove::build(3).unwrap()).unwrap();
    assert!(gb.is_over() && gb.winner() == Some(GamePlayer::Computer) && gb.winning_move == Some(GameMove(3)));
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
    assert!(gb.is_over() && gb.winner() == Some(GamePlayer::Computer) && gb.winning_move == Some(GameMove(6)));
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

    let mut gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

    gb.make_move(&GameMove::build(0).unwrap()).unwrap();
    assert!(!gb.is_over() && gb.winner() == None && gb.winning_move == None);
    assert_eq!(gb.max_streak(6, 2), 3);

    gb.make_move(&GameMove::build(6).unwrap()).unwrap();
    assert!(!gb.is_over() && gb.winner() == None && gb.winning_move == None);
    assert_eq!(gb.max_streak(0, 2), 2);
}

// #[test]
// fn cause_win_horizontal() {
//     let mut board = [[None; 7]; 6];
//     board[0][0] = Some(GamePlayer::Opponent);
//     board[0][1] = Some(GamePlayer::Opponent);
//     board[0][2] = Some(GamePlayer::Computer);
//     board[0][3] = Some(GamePlayer::Computer);
//     board[0][4] = Some(GamePlayer::Computer);

//     board[1][1] = Some(GamePlayer::Opponent);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(gb.will_cause_win(&GamePlayer::Computer, GameMove::build(5).unwrap()));

//     board[1][0] = Some(GamePlayer::Opponent);
//     board[1][1] = Some(GamePlayer::Opponent);
//     board[1][2] = Some(GamePlayer::Opponent);

//     board[0][4] = Some(GamePlayer::Computer);
//     board[0][5] = Some(GamePlayer::Computer);
//     board[0][6] = Some(GamePlayer::Computer);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(gb.will_cause_win(&GamePlayer::Opponent, GameMove::build(3).unwrap()));
// }

// #[test]
// fn cause_win_horizontal_split() {
//     let mut board = [[None; 7]; 6];
//     board[0][3] = Some(GamePlayer::Computer);
//     board[0][5] = Some(GamePlayer::Computer);
//     board[0][6] = Some(GamePlayer::Computer);

//     board[1][3] = Some(GamePlayer::Opponent);
//     board[1][5] = Some(GamePlayer::Opponent);
//     board[1][6] = Some(GamePlayer::Opponent);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(gb.will_cause_win(&GamePlayer::Computer, GameMove::build(4).unwrap()));
// }

// #[test]
// fn cause_win_horizontal_edge() {
//     let mut board = [[None; 7]; 6];

//     board[0][3] = Some(GamePlayer::Computer);
//     board[0][4] = Some(GamePlayer::Opponent);
//     board[0][5] = Some(GamePlayer::Computer);
//     board[0][6] = Some(GamePlayer::Computer);

//     board[1][3] = Some(GamePlayer::Computer);
//     board[1][4] = Some(GamePlayer::Computer);
//     board[1][5] = Some(GamePlayer::Computer);

//     board[2][3] = Some(GamePlayer::Opponent);
//     board[2][4] = Some(GamePlayer::Opponent);
//     board[2][5] = Some(GamePlayer::Opponent);

//     board[3][5] = Some(GamePlayer::Opponent);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(gb.will_cause_win(&GamePlayer::Computer, GameMove::build(6).unwrap()));
// }

// #[test]
// fn wont_cause_win_bad_horizontal_split() {
//     let mut board = [[None; 7]; 6];

//     board[0][3] = Some(GamePlayer::Computer);

//     board[0][5] = Some(GamePlayer::Opponent);
//     board[0][6] = Some(GamePlayer::Computer);

//     board[1][3] = Some(GamePlayer::Computer);
//     board[1][5] = Some(GamePlayer::Computer);
//     board[1][6] = Some(GamePlayer::Computer);

//     board[2][3] = Some(GamePlayer::Opponent);
//     board[2][5] = Some(GamePlayer::Opponent);
//     board[2][6] = Some(GamePlayer::Opponent);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(!gb.will_cause_win(&GamePlayer::Computer, GameMove::build(4).unwrap()));
// }

// #[test]
// fn cause_win_main_diag() {
//     let mut board = [[None; 7]; 6];

//     board[0][0] = Some(GamePlayer::Opponent);
//     board[0][1] = Some(GamePlayer::Computer);
//     board[0][2] = Some(GamePlayer::Opponent);
//     board[0][3] = Some(GamePlayer::Computer);

//     board[1][1] = Some(GamePlayer::Opponent);
//     board[1][2] = Some(GamePlayer::Computer);
//     board[1][3] = Some(GamePlayer::Opponent);

//     board[2][2] = Some(GamePlayer::Opponent);
//     board[2][3] = Some(GamePlayer::Computer);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(gb.will_cause_win(&GamePlayer::Opponent, GameMove::build(3).unwrap()))
// }

// #[test]
// fn cause_win_main_diag_split() {
//     let mut board = [[None; 7]; 6];

//     board[0][3] = Some(GamePlayer::Computer);
//     board[0][4] = Some(GamePlayer::Opponent);
//     board[0][5] = Some(GamePlayer::Opponent);
//     board[0][6] = Some(GamePlayer::Computer);

//     board[1][5] = Some(GamePlayer::Opponent);
//     board[1][6] = Some(GamePlayer::Computer);

//     board[2][5] = Some(GamePlayer::Computer);
//     board[2][6] = Some(GamePlayer::Opponent);

//     board[3][6] = Some(GamePlayer::Computer);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(gb.will_cause_win(&GamePlayer::Computer, GameMove::build(4).unwrap()))
// }

// #[test]
// fn wont_cause_win_main_diag_split() {
//     let mut board = [[None; 7]; 6];

//     board[0][3] = Some(GamePlayer::Opponent);
//     board[0][4] = Some(GamePlayer::Computer);
//     board[0][5] = Some(GamePlayer::Opponent);
//     board[0][6] = Some(GamePlayer::Computer);

//     board[1][5] = Some(GamePlayer::Opponent);
//     board[1][6] = Some(GamePlayer::Computer);

//     board[2][5] = Some(GamePlayer::Computer);
//     board[2][6] = Some(GamePlayer::Opponent);

//     board[3][6] = Some(GamePlayer::Computer);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(!gb.will_cause_win(&GamePlayer::Computer, GameMove::build(4).unwrap()))
// }

// #[test]
// fn cause_win_off_diag() {
//     let mut board = [[None; 7]; 6];

//     board[0][0] = Some(GamePlayer::Opponent);
//     board[0][1] = Some(GamePlayer::Computer);
//     board[0][2] = Some(GamePlayer::Opponent);
//     board[0][3] = Some(GamePlayer::Computer);

//     board[1][0] = Some(GamePlayer::Opponent);
//     board[1][1] = Some(GamePlayer::Opponent);
//     board[1][2] = Some(GamePlayer::Computer);

//     board[2][0] = Some(GamePlayer::Opponent);
//     board[2][1] = Some(GamePlayer::Computer);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(gb.will_cause_win(&GamePlayer::Computer, GameMove::build(0).unwrap()))
// }

// #[test]
// fn cause_win_off_diag_split() {
//     let mut board = [[None; 7]; 6];

//     board[0][0] = Some(GamePlayer::Opponent);
//     board[0][1] = Some(GamePlayer::Computer);
//     board[0][2] = Some(GamePlayer::Opponent);
//     board[0][3] = Some(GamePlayer::Computer);

//     board[1][0] = Some(GamePlayer::Opponent);
//     board[1][1] = Some(GamePlayer::Opponent);

//     board[2][0] = Some(GamePlayer::Opponent);
//     board[2][1] = Some(GamePlayer::Computer);

//     board[3][0] = Some(GamePlayer::Computer);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(gb.will_cause_win(&GamePlayer::Computer, GameMove::build(2).unwrap()))
// }

// #[test]
// fn wont_cause_win_off_diag_split() {
//     let mut board = [[None; 7]; 6];

//     board[0][3] = Some(GamePlayer::Opponent);
//     board[0][4] = Some(GamePlayer::Computer);
//     board[0][5] = Some(GamePlayer::Opponent);
//     board[0][6] = Some(GamePlayer::Opponent);

//     board[1][5] = Some(GamePlayer::Opponent);
//     board[1][6] = Some(GamePlayer::Computer);

//     board[2][5] = Some(GamePlayer::Computer);
//     board[2][6] = Some(GamePlayer::Opponent);

//     board[3][6] = Some(GamePlayer::Computer);

//     let gb = GameBoard::build(board).unwrap();

//     assert!(!gb.will_cause_win(&GamePlayer::Computer, GameMove::build(4).unwrap()))
// }
