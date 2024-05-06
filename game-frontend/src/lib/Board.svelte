<script>
  import { fade } from "svelte/transition";
  import GameButtons from "./GameButtons.svelte";
  import GameEndScreen from "./GameEndScreen.svelte";
  import Piece from "./Piece.svelte";
  import { gameIsOver, top } from "./game-lib";
  import { createEventDispatcher } from "svelte";
  import Waiting from "./Waiting.svelte";

  export let gameOverText;
  

  let dispatch = createEventDispatcher();
  let disabledButtons = Array(7).fill(false);
  let board = Array.from({ length: 6 }, () => Array(7).fill(0));
  let turns = 0;
  let pieces = [];
  let buttons;
  let isOver = false;
  let justStarted = true;
  let promise = Promise.resolve();
  setTimeout(() => (justStarted = false), 1000);

  export function waitOnPromise(newPromise){
    setTimeout(() => promise = newPromise, 150);
  }
  export function clear() {
    pieces = [];
    disabledButtons = Array(7).fill(false);
    board = Array.from({ length: 6 }, () => Array(7).fill(0));
    turns = 0;
    isOver = false;
    buttons.enable();
    promise = Promise.resolve();
  }

  /**
   * Disables the buttons which allow the user to play moves
   */
  export function disableButtons() {
    buttons.disable();
  }

  /**
   * Enables buttons which are do not correspond to a full column
   */
  export function enableButtons() {
    if (isOver) return;
    for (let i = 0; i < 7; i++) {
      // console.log(`Here: ${i}, disabled: ${disabledButtons[i]}`);
      buttons.setCol(i, disabledButtons[i]);
    }
  }

  /**
   * Returns the board. 1 is the first player, -1 is the second player, 0 is empty.
   */
  export function getBoard() {
    return board;
  }

  /**
   * Return a boolean representing if the game is over
   */
  export function gameOver() {
    return isOver;
  }

  /**
   * Plays a move and disables the all the buttons
   * @param {number} col
   */
  export function playMove(col) {
    buttons.disable();

    let row = top(col, board);
    if (row > 5) {
      errorEvent("Played Piece in full column");
    }
    if (row === 5) disabledButtons[col] = true;

    let player = turns % 2 === 0 ? 1 : -1;

    board[row][col] = player;
    placePiece(row, col, player === 1 ? "red" : "blue");
    turns++;

    let winner = gameIsOver(board);

    if (winner !== 0) {
      isOver = true;
      endGameEvent(`${winner == 1 ? "Red" : "Blue"} Wins!`);
      return;
    }

    if (turns == 42) {
      isOver = true;
      endGameEvent("Draw");
    }
  }

  function endGameEvent(winner) {
    dispatch("gameEnd", {
      message: winner,
    });
  }

  function errorEvent(msg = "") {
    dispatch("error", {
      message: msg,
    });
  }

  function buttonPressEvent(col) {
    dispatch("playerMove", {
      col: col,
    });
  }

  function placePiece(row, col, color) {
    pieces = [...pieces, { row, col, color }];
  }
</script>

<div class="grid">
  <div
    class="container gridItem"
    transition:fade={{ delay: 150, duration: 300 }}
  >
    <!-- some svg code adapted from https://rossta.net/blog/connect-four-with-svg-pattern-masking.html -->
    <svg viewBox="0 0 700 700" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <pattern
          id="cell-pattern"
          patternUnits="userSpaceOnUse"
          width="100"
          height="100"
        >
          <circle cx="50" cy="50" r="45" fill="black"></circle>
        </pattern>
        <mask id="cell-mask">
          <rect width="700" height="700" fill="white"></rect>
          <rect width="700" height="700" fill="url(#cell-pattern)"></rect>
        </mask>
      </defs>
      {#each pieces as { row, col, color }, i (i)}
        <Piece {row} {col} {color}></Piece>
      {/each}

      <rect
        x="0"
        y="100"
        height="600"
        width="700"
        fill="#303030"
        mask="url(#cell-mask)"
      ></rect>
    </svg>
    <GameButtons
      bind:this={buttons}
      on:buttonClick={(e) => {
        playMove(e.detail.col);
        buttonPressEvent(e.detail.col);
      }}
    ></GameButtons>
  </div>
  {#await promise}
    {#if !justStarted}
      <div class="gridItem" transition:fade={{ delay: 150, duration: 300 }}>
        <Waiting />
      </div>
    {/if}
  {/await}
  {#if isOver}
    <div class="gridItem" transition:fade={{ delay: 150, duration: 300 }}>
      <GameEndScreen gameText={gameOverText} on:menuClicked on:replayClicked />
    </div>
  {/if}
</div>

<style>
  svg {
    width: max(min(calc(100vh - 300px), min(1200px, 100vw)), 300px);
    height: max(min(calc(100vh - 300px), min(1200px, 100vw)), 300px);
  }
  div.container {
    display: flex;
    flex-direction: column;
  }
  .gridItem {
    grid-row: 1;
    grid-column: 1;
  }
  div.grid {
    display: grid;
    justify-items: center;
    align-items: center;
  }
</style>
