<script>
  import Game from './lib/Game.svelte'
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
      game.startGame()}}/>

  {/if}
  
  {#if winnerText !== ""}
      <h3>{winnerText === "Draw" ? "Draw" : winnerText + " Wins"}</h3>

  {/if}

  <Game bind:this = {game} on:gameover= {(e) => {
    game.endGame();
    gameInProg = false;
    winnerText = e.detail.winner;
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
