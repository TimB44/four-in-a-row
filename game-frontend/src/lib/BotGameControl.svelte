<script>
  import Board from "./Board.svelte";
  import GameEndScreen from "./GameEndScreen.svelte";
  import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";

  export let botDiff = "easy";
  export let playerIsFirst = true;

  let dispatch = createEventDispatcher();
  let gameIsOver = false;
  let gameOverText = "";
  let board;
  onMount(() => {
    if (!playerIsFirst) {
      board.disableButtons();
      playAIMove();
    }
  });

  async function playAIMove() {
    let promise = fetch("/ai", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        board: board.getBoard(),
        difficulty: botDiff,
      }),
    });

    board.waitOnPromise(promise);
    let resp = await promise;
    if (!resp.ok) {
      console.log("Could not get AI move from server");
      errorEvent("Could not get AI move from server");
      return;
    }
    let json = await resp.json();

    let move = json["move"];
    board.playMove(move);
    board.enableButtons();
  }

  function errorEvent(msg = "") {
    dispatch("error", {
      message: msg,
    });
  }
</script>

<Board
  {gameOverText}
  bind:this={board}
  on:menuClicked
  on:error
  on:replayClicked={() => {
    board.clear();
  }}
  on:playerMove={() => {
    if (!board.gameOver()) playAIMove();
  }}
  on:gameEnd={(e) => {
    gameOverText = e.detail.message;
    gameIsOver = true;
  }}
/>

<style>
</style>
