<script>
  import { createEventDispatcher } from "svelte";
  import { gameIsOver, top } from "./game-lib";
  import Board from "./Board.svelte";
  import GameButtons from "./GameButtons.svelte";
  import { onMount } from "svelte";

  export let playerIsFirst = true;
  export let gameId = -1;
  const dispatch = createEventDispatcher();

  let disabled = Array(7).fill(false);
  let board = Array.from({ length: 6 }, () => Array(7).fill(0));
  let turns = 0;
  let buttons;
  let visualBoard;
  console.log("in multiplayer game control.");
  console.log(`player is first: ${playerIsFirst}`);

  // If no gameID given then fetch one form the server and give player link

  if (gameId == -1) {
    let promise = fetch("/multiplayer/create_game", { method: "POST" });
    promise
      .then((resp) => resp.json().then((json) => (gameId = json["id"])))
      .catch((res) => {
        console.log(res);
        throwError();
      });
  }

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
  function throwError() {
    dispatch("gameEnd", { error: true });
  }



  function playMove(col) {
    buttons.disable();

    let row = top(col, board);

    if (row === 5) disabled[col] = true;

    //first player is 1, second -1
    let player = playerIsFirst ? 1 : -1;

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
      return;
    }
    if (player == 1) {
      getMove();
    } else {
      disabled.forEach((val, i) => buttons.setCol(i, val));
    }
  }
  async function getMove() {
    let start = Date.now();
    let promise = fetch("/ai", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        board: board,
        // difficulty: botDiff,
        first_player: playerIsFirst ? 1 : -1,
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
      getMove();
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
  <!-- {#if error}
    <h1>Error</h1>
  {:else}
    <Board bind:this={visualBoard} />
    <GameButtons
      bind:this={buttons}
      on:buttonClick={(e) => {
        playMove(e.detail.col);
      }}
    ></GameButtons>
  {/if} -->
</div>

<style>
</style>
