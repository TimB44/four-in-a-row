/**
 *
 * @param {number[][]} board
 * @returns {number}
 */
export function gameIsOver(board) {
    let winner = verticalWin(board);
    if (winner !== 0) return winner;

    winner = horizontalWin(board);
    if (winner !== 0) return winner;

    winner = upDiagonalWin(board);
    if (winner !== 0) return winner;

    winner = downDiagonalWin(board);
    return winner;
}

/**
 * This function will
 * @param {number} col
 * @param {number[][]} board
 * @returns
 */
export function top(col, board) {
    for (let i = 0; i < board.length; i++) {
        if (board[i][col] === 0) {
            return i;
        }
    }
    return 6;
}

/**
 *
 * @param {number[][]} board
 * @returns {number}
 */
function verticalWin(board) {
    for (let row = 0; row < 3; row++) {
        colLoop: for (let col = 0; col < 7; col++) {
            let player = board[row][col];
            if (player === 0) continue;

            for (let up = 1; up < 4; up++) {
                if (board[row + up][col] !== player) continue colLoop;
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
    for (let row = 0; row < 6; row++) {
        colLoop: for (let col = 0; col < 4; col++) {
            let player = board[row][col];

            if (player === 0) continue;

            for (let i = 1; i < 4; i++) {
                if (board[row][col + i] !== player) continue colLoop;
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
function upDiagonalWin(board) {
    for (let row = 0; row < 3; row++) {
        colLoop: for (let col = 0; col < 4; col++) {
            let player = board[row][col];

            if (player === 0) continue;

            for (let i = 1; i < 4; i++) {
                if (board[row + i][col + i] !== player) continue colLoop;
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
function downDiagonalWin(board) {
    for (let row = 3; row < 6; row++) {
        colLoop: for (let col = 0; col < 4; col++) {
            let player = board[row][col];

            if (player === 0) continue;

            for (let i = 1; i < 4; i++) {
                if (board[row - i][col + i] !== player) continue colLoop;
            }
            return player;
        }
    }

    return 0;
}
