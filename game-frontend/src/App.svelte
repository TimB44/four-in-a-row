<script>
  import LocalGameControl from './lib/LocalGameControl.svelte';
  import GameSelector from './lib/GameSelector.svelte'

  let gameInProg = false;
  let game;
  let winnerText = "";

</script>
<head>
  <title>Four In a Row</title>
</head>
<main>
  

  <h1>Four In a Row</h1>
  
  {#if !gameInProg}
    <GameSelector on:gameStart = {(e) => {
      gameInProg = true;
      game.start()}}/>

  {/if}
  
  {#if winnerText !== ""}
      <dialog>{winnerText}</dialog>

  {/if}

  <LocalGameControl bind:this = {game} on:gameEnd= {(e) => {
    gameInProg = false;
    winnerText = e.detail.message;
    }}/>

</main>

<style>
 main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }
</style>
