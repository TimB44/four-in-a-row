<script>
  import Board from "./Board.svelte";
  let board;
  let gameOverText = "";
</script>

<Board
  {gameOverText}
  on:replayClicked={() => {
    board.clear();
    board.enableButtons();
  }}
  on:menuClicked
  bind:this={board}
  on:playerMove={() => board.enableButtons()}
  on:error
  on:gameEnd={(e) => {
    gameOverText = e.detail.message;
  }}
/>

<div></div>

<style>
</style>
