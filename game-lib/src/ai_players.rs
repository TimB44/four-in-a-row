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
    minimax(board, 8)
}


#[cfg(test)]
mod tests {
    use crate::{ai_players::*, board::{GameBoard, GameMove, GamePlayer}};

    #[test]
    fn play_easy_block(){
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::Opponent);
        board[1][0] = Some(GamePlayer::Opponent);
        board[2][0] = Some(GamePlayer::Opponent);

        board[0][4] = Some(GamePlayer::Computer);
        board[1][4] = Some(GamePlayer::Computer);
        board[0][5] = Some(GamePlayer::Computer);
        
        let gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

        assert_eq!(play_easy(gb), GameMove::build(0));
    }

    #[test]
    fn play_easy_win(){
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::Opponent);
        board[1][0] = Some(GamePlayer::Opponent);
        board[2][0] = Some(GamePlayer::Opponent);

        board[0][4] = Some(GamePlayer::Computer);
        board[1][4] = Some(GamePlayer::Computer);
        board[2][4] = Some(GamePlayer::Computer);
        
        let gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

        assert_eq!(play_easy(gb), GameMove::build(4));
    }

    #[test]
    fn play_med_block(){
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::Opponent);
        board[1][0] = Some(GamePlayer::Opponent);
        board[2][0] = Some(GamePlayer::Opponent);

        board[0][4] = Some(GamePlayer::Computer);
        board[1][4] = Some(GamePlayer::Computer);
        board[0][5] = Some(GamePlayer::Computer);
        
        let gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

        assert_eq!(play_med(gb), GameMove::build(0));
    }

    #[test]
    fn play_med_win(){
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::Opponent);
        board[1][0] = Some(GamePlayer::Opponent);
        board[2][0] = Some(GamePlayer::Opponent);

        board[0][4] = Some(GamePlayer::Computer);
        board[1][4] = Some(GamePlayer::Computer);
        board[2][4] = Some(GamePlayer::Computer);
        
        let gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

        assert_eq!(play_med(gb), GameMove::build(4));
    }

    #[test]
    fn play_hard_block(){
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::Opponent);
        board[1][0] = Some(GamePlayer::Opponent);
        board[2][0] = Some(GamePlayer::Opponent);

        board[0][4] = Some(GamePlayer::Computer);
        board[1][4] = Some(GamePlayer::Computer);
        board[0][5] = Some(GamePlayer::Computer);
        
        let gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

        assert_eq!(play_hard(gb), GameMove::build(0));
    }

    #[test]
    fn play_hard_win(){
        let mut board = [[None; 7]; 6];
        board[0][0] = Some(GamePlayer::Opponent);
        board[1][0] = Some(GamePlayer::Opponent);
        board[2][0] = Some(GamePlayer::Opponent);

        board[0][4] = Some(GamePlayer::Computer);
        board[1][4] = Some(GamePlayer::Computer);
        board[2][4] = Some(GamePlayer::Computer);
        
        let gb = GameBoard::build(board, GamePlayer::Computer).unwrap();

        assert_eq!(play_hard(gb), GameMove::build(4));
    }


    #[test]
    fn play_hard_finishes(){
        let mut board = [[None; 7]; 6];
        board[0][3] = Some(GamePlayer::Opponent);
        board[0][4] = Some(GamePlayer::Computer);

        let gb = GameBoard::build(board, GamePlayer::Computer).unwrap();
        println!("{:?}", play_hard(gb).unwrap());
    }


    #[test]
    fn play_hard_mid_game() {
        let mut board = [[None; 7]; 6];

        board[0][2] = Some(GamePlayer::Opponent);
        board[0][3] = Some(GamePlayer::Opponent);

        board[1][3] = Some(GamePlayer::Computer);

        let gb = GameBoard::build(board, GamePlayer::Opponent).unwrap();
        print!("{:?}", play_hard(gb));
    }
}
