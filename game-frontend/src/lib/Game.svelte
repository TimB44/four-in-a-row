<script>
  import { createEventDispatcher } from "svelte";
  import { gameIsOver } from "./game-lib";
  import { gameSettings } from "./stores";
  import { onDestroy, onMount } from "svelte";
  import Board from "./Board.svelte";

  const dispatch = createEventDispatcher();
  let inGame = false;

  let firstPlayerIsRed = true;
  let isOnePlayer = true;
  let aiDifficulty = "easy";

  const unsubscribe = gameSettings.subscribe((value) => {
    firstPlayerIsRed = value.firstPlayerIsRed;
    isOnePlayer = value.isOnePlayer;
    aiDifficulty = value.aiDifficulty;
  });

  onDestroy(() => {
    unsubscribe();
  });

  onMount(() => {
    endGame();
  })

  let turns = 0;
  let buttons = [];
  let disabled = Array(7).fill(false);
  let board = Array.from({ length: 6 }, () => Array(7).fill(0));
  let visualBoard;



  export function startGame() {
    disabled = Array(7).fill(false);
    board = Array.from({ length: 6 }, () => Array(7).fill(0));
    turns = 0;
    inGame = true;
    buttons.forEach((e) => e.disabled = false);
    visualBoard.clear();

    
  }

  export function endGame() {
    inGame = false;
    buttons.forEach((e) => e.disabled = true);
  }

  /**
   * Ends the game and triggers and event to notify its parent
   * @param {String} winner
   */
  function gameOver(winner) {
    dispatch("gameover", {
      winner: winner,
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
    buttons.forEach((element) => {
      element.disabled = true;
    });
    let row = top(col);

    if (row === 5) disabled[col] = true;

    let player = turns % 2 === 0 ? "red" : "blue";

    board[row][col] = !isOnePlayer && player === "blue" ? -1 : 1;
    turns++;

    visualBoard.placePiece(row, col, player);

    let winner = gameIsOver(board);

    if (winner !== 0) {
      gameOver(winner == 1 ? "Red" : "Blue");
      return;
    }

    if (turns == 42) {
      gameOver("Draw");
    }

    if (!isOnePlayer) {
      for (let i = 0; i < disabled.length; i++) {
        buttons[i].disabled = disabled[i];
      }
    } else {
      //todo
    }

    board = board;
  }
</script>

<div class={inGame ? "" : "dark"}>
  <div>
    <Board bind:this={visualBoard} />
  </div>
  <div>
    {#each Array(7) as _, i (i)}
      <button bind:this={buttons[i]} on:click={() => playMove(i)}></button>
    {/each}
  </div>
</div>

<style>
  .dark * {
    opacity: 0.5; /* Set the opacity to make all child elements clear */
  }
</style>
