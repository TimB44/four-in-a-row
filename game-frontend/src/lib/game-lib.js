import { randomBytes } from "crypto";

/**
 * 
 * @param {number[][]} board 
 * @returns {number}
 */
export function gameIsOver(board) {
    let winner = verticalWin(board);
    if( winner !== 0 )
        return winner;
    winner = horizontalWin(board);
    if( winner !== 0 )
        return winner;

    winner = upDiagonalWin(board);
    if( winner !== 0 )
        return winner;

    winner = downDiagonalWin(board);

    return winner;
}
/**
 * 
 * @param {number[][]} board 
 * @returns {number}
 */
function verticalWin(board) {
    for(let row = 0; row < 3; row++){
        colLoop: for(let col = 0; col < 7; col++){
            let player = board[row][col];
            if(player === 0) continue;

            for(let up = 1; up < 4; up++) {
                if(board[row + up][col] !== player)
                    continue colLoop;
            }

            return player;
        }
    }

    return 0; 
}

/**
 * 
 * @param {number[][]} board 
 * @returns {number}
 */
function horizontalWin(board) {
    throw new Error("Function not implemented.");
}

/**
 * 
 * @param {number[][]} board 
 * @returns {number}
 */
function upDiagonalWin(board) {
    throw new Error("Function not implemented.");
}

/**
 * 
 * @param {number[][]} board 
 * @returns {number}
 */
function downDiagonalWin(board) {
    throw new Error("Function not implemented.");
}

