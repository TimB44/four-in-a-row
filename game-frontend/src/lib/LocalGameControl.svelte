<script>
  import Board from "./Board.svelte";
  import { createEventDispatcher } from "svelte";
  import GameEndScreen from "./GameEndScreen.svelte";

  const dispatch = createEventDispatcher();
  let board;
  let gameIsOver = false;
  let gameOverText = "";

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

  function replayClicked() {
    gameIsOver = false;
    board.clear();
  }
</script>

{#if gameIsOver}
  <GameEndScreen
    on:replayClicked={replayClicked}
    on:menuClicked
    gameText={gameOverText}
  />
{/if}
<Board
  bind:this={board}
  on:playerMove={() => board.enableButtons()}
  on:error
  on:gameEnd={(e) => {
    gameOverText = e.detail.message;
    gameIsOver = true;
  }}
/>

<div></div>

<style>
</style>
