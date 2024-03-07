<script>
  import LocalGameControl from "./lib/LocalGameControl.svelte";
  import GameSelector from "./lib/GameSelector.svelte";
  import { gameSettings } from "./lib/stores";
  import BotGameControl from "./lib/BotGameControl.svelte";

  let gameInProg = false;
  let mode;
  gameSettings.subscribe((gs) => (mode = gs.mode));
  let winnerText = "";
  let game;

  $: console.log(winnerText);

  function gameEnd(e) {
    if (e.detail.error) {
      winnerText = `Error: ${e.detail.message}`;
    } else {
      winnerText = e.detail.message;
    }

    gameInProg = false;
  }
  $: console.log(`Mode: ${mode}`);
</script>

<head>
  <title>Four In a Row</title>
</head>
<main>
  <h1>Four In a Row</h1>

  {#if !gameInProg}
    <GameSelector
      on:gameStart={(e) => {
        gameInProg = true;
        game.start();
      }}
    />
  {/if}

  {#if winnerText !== ""}
    <p>{winnerText}</p>
  {/if}
  {#if mode === 1}
    <LocalGameControl bind:this={game} on:gameEnd={gameEnd} />
  {:else if mode == 2}
    <BotGameControl
      bind:this={game}
      on:gameEnd={gameEnd}
      botDiff={$gameSettings.botSettings.botDiff}
      playerIsFirst={$gameSettings.botSettings.playerIsFirst}
    />
  {:else}
    <p>Error Please Refresh Page</p>
  {/if}
</main>

<style>
  main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }
</style>
