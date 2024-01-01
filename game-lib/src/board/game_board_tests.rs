use crate::board::*;

#[test]
fn cause_win_vertical_left() {
    let mut board = [[None; 7]; 6];
    board[0][0] = Some(Players::Opp);
    board[1][0] = Some(Players::Opp);
    board[2][0] = Some(Players::Opp);

    board[0][1] = Some(Players::You);
    board[1][1] = Some(Players::You);
    board[2][1] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::Opp, GameMove::build(0).unwrap()))
}

#[test]
fn cause_win_vertical_mid() {
    let mut board = [[None; 7]; 6];
    board[0][3] = Some(Players::You);
    board[1][3] = Some(Players::You);
    board[2][3] = Some(Players::You);

    board[0][0] = Some(Players::Opp);
    board[1][0] = Some(Players::Opp);
    board[2][0] = Some(Players::Opp);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::You, GameMove::build(3).unwrap()))
}

#[test]
fn cause_win_vertical_right() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(Players::You);
    board[1][6] = Some(Players::You);
    board[2][6] = Some(Players::You);

    board[0][0] = Some(Players::Opp);
    board[1][0] = Some(Players::Opp);
    board[2][0] = Some(Players::Opp);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::You, GameMove::build(6).unwrap()))
}

#[test]
fn wont_cause_win_not_same_player() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(Players::You);
    board[1][6] = Some(Players::Opp);
    board[2][6] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(!gb.will_cause_win(&Players::You, GameMove::build(6).unwrap()))
}

#[test]
fn wont_cause_win_3_streak() {
    let mut board = [[None; 7]; 6];
    board[0][6] = Some(Players::You);
    board[1][6] = Some(Players::You);

    board[0][0] = Some(Players::Opp);

    let gb = GameBoard::build(board).unwrap();

    assert!(!gb.will_cause_win(&Players::You, GameMove::build(6).unwrap()))
}

#[test]
fn cause_win_horizontal() {
    let mut board = [[None; 7]; 6];
    board[0][0] = Some(Players::Opp);
    board[0][1] = Some(Players::Opp);
    board[0][2] = Some(Players::You);
    board[0][3] = Some(Players::You);
    board[0][4] = Some(Players::You);

    board[1][1] = Some(Players::Opp);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::You, GameMove::build(5).unwrap()));

    board[1][0] = Some(Players::Opp);
    board[1][1] = Some(Players::Opp);
    board[1][2] = Some(Players::Opp);

    board[0][4] = Some(Players::You);
    board[0][5] = Some(Players::You);
    board[0][6] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::Opp, GameMove::build(3).unwrap()));
}

#[test]
fn cause_win_horizontal_split() {
    let mut board = [[None; 7]; 6];
    board[0][3] = Some(Players::You);
    board[0][5] = Some(Players::You);
    board[0][6] = Some(Players::You);

    board[1][3] = Some(Players::Opp);
    board[1][5] = Some(Players::Opp);
    board[1][6] = Some(Players::Opp);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::You, GameMove::build(4).unwrap()));
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

    board[2][3] = Some(Players::Opp);
    board[2][4] = Some(Players::Opp);
    board[2][5] = Some(Players::Opp);

    board[3][5] = Some(Players::Opp);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::You, GameMove::build(6).unwrap()));
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

    board[2][3] = Some(Players::Opp);
    board[2][5] = Some(Players::Opp);
    board[2][6] = Some(Players::Opp);

    let gb = GameBoard::build(board).unwrap();

    assert!(!gb.will_cause_win(&Players::You, GameMove::build(4).unwrap()));
}

#[test]
fn cause_win_main_diag() {
    let mut board = [[None; 7]; 6];

    board[0][0] = Some(Players::Opp);
    board[0][1] = Some(Players::You);
    board[0][2] = Some(Players::Opp);
    board[0][3] = Some(Players::You);

    board[1][1] = Some(Players::Opp);
    board[1][2] = Some(Players::You);
    board[1][3] = Some(Players::Opp);

    board[2][2] = Some(Players::Opp);
    board[2][3] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::Opp, GameMove::build(3).unwrap()))
}

#[test]
fn cause_win_main_diag_split() {
    let mut board = [[None; 7]; 6];

    board[0][3] = Some(Players::You);
    board[0][4] = Some(Players::Opp);
    board[0][5] = Some(Players::Opp);
    board[0][6] = Some(Players::You);

    board[1][5] = Some(Players::Opp);
    board[1][6] = Some(Players::You);

    board[2][5] = Some(Players::You);
    board[2][6] = Some(Players::Opp);

    board[3][6] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::You, GameMove::build(4).unwrap()))
}

#[test]
fn wont_cause_win_main_diag_split() {
    let mut board = [[None; 7]; 6];

    board[0][3] = Some(Players::Opp);
    board[0][4] = Some(Players::You);
    board[0][5] = Some(Players::Opp);
    board[0][6] = Some(Players::You);

    board[1][5] = Some(Players::Opp);
    board[1][6] = Some(Players::You);

    board[2][5] = Some(Players::You);
    board[2][6] = Some(Players::Opp);

    board[3][6] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(!gb.will_cause_win(&Players::You, GameMove::build(4).unwrap()))
}

#[test]
fn cause_win_off_diag() {
    let mut board = [[None; 7]; 6];

    board[0][0] = Some(Players::Opp);
    board[0][1] = Some(Players::You);
    board[0][2] = Some(Players::Opp);
    board[0][3] = Some(Players::You);

    board[1][0] = Some(Players::Opp);
    board[1][1] = Some(Players::Opp);
    board[1][2] = Some(Players::You);

    board[2][0] = Some(Players::Opp);
    board[2][1] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::You, GameMove::build(0).unwrap()))
}

#[test]
fn cause_win_off_diag_split() {
    let mut board = [[None; 7]; 6];

    board[0][0] = Some(Players::Opp);
    board[0][1] = Some(Players::You);
    board[0][2] = Some(Players::Opp);
    board[0][3] = Some(Players::You);

    board[1][0] = Some(Players::Opp);
    board[1][1] = Some(Players::Opp);

    board[2][0] = Some(Players::Opp);
    board[2][1] = Some(Players::You);

    board[3][0] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(gb.will_cause_win(&Players::You, GameMove::build(2).unwrap()))
}

#[test]
fn wont_cause_win_off_diag_split() {
    let mut board = [[None; 7]; 6];

    board[0][3] = Some(Players::Opp);
    board[0][4] = Some(Players::You);
    board[0][5] = Some(Players::Opp);
    board[0][6] = Some(Players::Opp);

    board[1][5] = Some(Players::Opp);
    board[1][6] = Some(Players::You);

    board[2][5] = Some(Players::You);
    board[2][6] = Some(Players::Opp);

    board[3][6] = Some(Players::You);

    let gb = GameBoard::build(board).unwrap();

    assert!(!gb.will_cause_win(&Players::You, GameMove::build(4).unwrap()))
}
