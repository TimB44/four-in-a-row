<script>
  import { createEventDispatcher } from "svelte";
  import { gameIsOver } from "./game-lib";
  import Board from "./Board.svelte";
  import GameButtons from "./GameButtons.svelte";

  const dispatch = createEventDispatcher();

  let disabled = Array(7).fill(false);
  let board = Array.from({ length: 6 }, () => Array(7).fill(0));
  let turns = 0;
  let buttons;
  let visualBoard;

  /**
   * Ends the game and triggers and event to notify its parent
   * @param {String} winner
   */
  function endGame(winner) {
    dispatch("gameEnd", {
      message: winner,
      error: false,
    });
  }

  function top(col) {
    for (let i = 0; i < board.length; i++) {
      if (board[i][col] === 0) {
        return i;
      }
    }
    return 6;
  }

  function playMove(col) {
    for (let i = 0; i < 7; i++) {
      buttons.setCol(i, true);
    }

    let row = top(col);

    if (row === 5) disabled[col] = true;

    let player = turns % 2 === 0 ? 1 : -1;

    board[row][col] = player;
    visualBoard.placePiece(row, col, player === 1 ? "red" : "blue");
    turns++;

    let winner = gameIsOver(board);

    if (winner !== 0) {
      endGame(`${winner == 1 ? "Red" : "Blue"} Wins!`);
      return;
    }

    if (turns == 42) {
      endGame("Draw");
    }

    for (let i = 0; i < 7; i++) {
      buttons.setCol(i, disabled[i]);
    }
  }

  export function start() {
    clear();
  }

  // Clears the board to the state it is when first created
  export function clear() {
    disabled.fill(false);
    board.forEach((row) => row.fill(0));
    turns = 0;
    visualBoard.clear();
    buttons.enable();
  }

  // Starts a new game with some state shared between sessions
  // In this game setting it is the same as start
  export function restart() {
    start();
  }
</script>

<div>
  <Board bind:this={visualBoard} />
  <GameButtons
    bind:this={buttons}
    on:buttonClick={(e) => {
      playMove(e.detail.col);
    }}
  ></GameButtons>
</div>

<style>
</style>
