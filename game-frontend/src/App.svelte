<script>
  import Game from './lib/Game.svelte'
  import GameSelector from './lib/GameSelector.svelte'

  let gameInProg = false;
  let game;

</script>

<main>

  <h1>Four In a Row</h1>
  
  {#if !gameInProg}
    <GameSelector on:gameStart = {(e) => {
      gameInProg = true;
      game.startGame()}}/>
  {/if}  

  <Game bind:this = {game} on:gameover= {(e) => {
    game.endGame();
    gameInProg = false;
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
