<script>
  import { createEventDispatcher } from "svelte";
  import Board from "./Board.svelte";
  import { onMount } from "svelte";

  export let playerIsFirst;
  export let gameId;
  const dispatch = createEventDispatcher();

  let board;
  let gameOverText = "";

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
      console.error(await resp.blob());
      errorEvent("Could not get opponent move");
    }
    if (!board.gameOver()) {
      getOpponentMove();
    }
  }

  async function getOpponentMove() {
    let promise = fetch("/multiplayer/get_board", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        game_id: gameId,
        current_board: board.getBoard(),
      }),
    });
    board.waitOnPromise(promise);
    let resp = await promise;

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

  async function resetGame() {
    let promise = fetch("/multiplayer/reset_game", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        game_id: gameId,
      }),
    });
    board.clear();
    board.waitOnPromise(promise);
    let resp = await promise;

    if (!resp.ok) {
      errorEvent("Could not reset game");
      return;
    }
    
    if (playerIsFirst) {
      board.enableButtons();
    } else {
      board.disableButtons();
      getOpponentMove();
    }
  }
</script>

<Board
  {gameOverText}
  bind:this={board}
  on:error
  on:menuClicked
  on:replayClicked={resetGame}
  on:playerMove={(e) => sendNewMove(e.detail.col)}
  on:gameEnd={(e) => {
    gameOverText = e.detail.message;
  }}
/>
