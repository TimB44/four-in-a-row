<script>
  import { createEventDispatcher } from "svelte";
  import Board from "./Board.svelte";
  import { onMount } from "svelte";
  import GameEndScreen from "./GameEndScreen.svelte";

  export let playerIsFirst = true;
  export let gameId = -1;
  const dispatch = createEventDispatcher();

  let board;
  let gameIsOver = false;
  let gameOverText = "";

  console.log(playerIsFirst);
  console.log(gameId);

  onMount(() => {
    if (playerIsFirst) {
      board.enableButtons();
    } else {
      board.disableButtons();
      getOpponentMove();
    }
  });

  function errorEvent(msg = "") {
    dispatch("error", {
      message: msg,
    });
  }

  async function sendNewMove(col) {
    let resp = await fetch("/multiplayer/make_move", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        game_move: col,
        is_first: playerIsFirst,
        game_id: gameId,
      }),
    });

    if (!resp.ok) {
      console.log(await resp.blob());
      errorEvent("Could not get opponent move");
    }
    if (!board.gameOver()) {
      getOpponentMove();
    }
  }
  async function getOpponentMove() {
    let resp = await fetch("/multiplayer/get_board", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        game_id: gameId,
        // difficulty: botDiff,
        current_board: board.getBoard(),
      }),
    });

    if (!resp.ok) {
      errorEvent("Could not get opponents move");
      return;
    }

    let json = await resp.json();

    let newBoard = json["board"];
    let newMoves = extractNewMoves(board.getBoard(), newBoard);
    if (newMoves.length !== 1) {
      errorEvent("Unexpected moves from server");
      return;
    }

    board.playMove(newMoves[0]);
    board.enableButtons();
  }

  export function extractNewMoves(oldBoard, newBoard) {
    let moves = [];
    for (let col = 0; col < 7; col++) {
      for (let row = 0; row < 6; row++) {
        if (newBoard[row][col] !== oldBoard[row][col]) {
          if (oldBoard[row][col] !== 0) {
            return [];
          }
          moves.push(col);
        }
      }
    }
    return moves;
  }
</script>

{#if gameIsOver}
  <!-- TODO add some http endpoint to reset a game that is over -->
  <GameEndScreen
    on:replayClicked={() => {
      gameIsOver = false;
      board.clear();
    }}
    on:menuClicked
    gameText={gameOverText}
  />
{/if}
<Board
  bind:this={board}
  on:playerMove={(e) => sendNewMove(e.detail.col)}
  on:error
  on:gameEnd={(e) => {
    gameOverText = e.detail.message;
    gameIsOver = true;
  }}
/>

<style>
</style>
