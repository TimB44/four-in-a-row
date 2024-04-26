<script>
  import LocalGameControl from "./lib/LocalGameControl.svelte";
  import GameSelector from "./lib/GameSelector.svelte";
  import { gameSettings } from "./lib/stores";
  import BotGameControl from "./lib/BotGameControl.svelte";
  import MultiplayerControl from "./lib/MultiplayerControl.svelte";
  import { fade } from "svelte/transition";
  let gameInProg = false;
  let mode;
  gameSettings.subscribe((gs) => (mode = gs.mode));
  let winnerText = "";
  let game;

  function gameEnd(e) {
    if (e.detail.error) {
      winnerText = `Error: ${e.detail.message}`;
    } else {
      winnerText = e.detail.message;
    }

    gameInProg = false;
  }
</script>

<head>
  <title>Four In a Row</title>
</head>
<main>
  <h1>Four In a Row</h1>

  {#if !gameInProg}
    <div transition:fade={{ delay: gameInProg ? 1000 : 0, duration: 600 }}>
      <GameSelector
        on:gameStart={(e) => {
          gameInProg = true;
          game.start();
        }}
      />
    </div>
  {/if}
  {#if winnerText !== ""}
    <p>{winnerText}</p>
  {/if}
  {#if mode === 1 && gameInProg}
    <div transition:fade={{ delay: gameInProg ? 0 : 1000, duration: 600 }}>
      <BotGameControl
        bind:this={game}
        on:gameEnd={gameEnd}
        botDiff={$gameSettings.modeSettings.botDiff}
        playerIsFirst={$gameSettings.modeSettings.playerIsFirst}
      />
    </div>
  {:else if mode === 2 && gameInProg}
    <div transition:fade={{ delay: gameInProg ? 0 : 1000, duration: 600 }}>
      <LocalGameControl bind:this={game} on:gameEnd={gameEnd} />
    </div>
  {:else if mode === 3 && gameInProg}
    <div transition:fade={{ delay: gameInProg ? 0 : 1000, duration: 600 }}>
      <MultiplayerControl
        bind:this={game}
        on:gameEnd(gameEnd)
        playerIsFirst={$gameSettings.modeSettings["playerIsFirst"]}
        gameId={$gameSettings.modeSettings["id"]}
      />
    </div>
  {/if}
</main>

<style>
  main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;
  }
</style>
