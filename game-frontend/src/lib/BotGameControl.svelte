<script>
  import { createEventDispatcher } from "svelte";
  import { gameIsOver } from "./game-lib";
  import Board from "./Board.svelte";
  import GameButtons from "./GameButtons.svelte";
  import { onMount } from "svelte";

  export let botDiff = "easy";
  export let playerIsFirst = true;

  // console.log(botDiff);
  // console.log(playerIsFirst);

  const dispatch = createEventDispatcher();

  let disabled = Array(7).fill(false);
  let board = Array.from({ length: 6 }, () => Array(7).fill(0));
  let turns = 0;
  let buttons;
  let visualBoard;

  onMount(() => {
    buttons.disable();
  });

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
    buttons.disable();

    let row = top(col);

    if (row === 5) disabled[col] = true;


    let player = turns % 2 === 0 ? 1 : -1;

    board[row][col] = player;
    visualBoard.placePiece(row, col, player == 1 ? "red" : "blue");
    turns++;

    let winner = gameIsOver(board);

    if (winner !== 0) {
      endGame(`${winner == 1 ? "Red" : "Blue"} Wins!`);
      return;
    }

    if (winner || turns == 42) {
      endGame("Draw");
      return;
    }
    if ((playerIsFirst && turns % 2 == 1) || (!playerIsFirst && turns % 2 == 0)) {
      playAIMove();
    } else {
      disabled.forEach((val, i) => buttons.setCol(i, val));
    }
  }
  async function playAIMove() {
    console.log(botDiff);
    let start = Date.now();
    let promise = fetch("/ai", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        board: board,
        difficulty:  botDiff,
      }),
    });

    let resp = await promise;

    let json = await resp.json();

    let move = json["move"];
    let dur = Date.now() - start;
    if (dur < 250) {
      setTimeout(() => {
        playMove(move);
      }, 250 - dur);
    } else {
      playMove(move);
    }
  }

  //Starts a new game session
  export function start() {
    if (!playerIsFirst) {
      buttons.disable();
      playAIMove();
    } else {
      buttons.enable();
    }
  }

  // Clears the board to the state it is when first created
  export function clear() {
    disabled.fill(false);
    board.forEach((row) => row.fill(0));
    turns = 0;
    visualBoard.clear();
    buttons.disable();
  }

  // Starts a new game with some state shared between sessions
  // In this game setting it is the same as start
  export function restart() {
    clear();
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
