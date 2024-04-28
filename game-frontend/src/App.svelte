<script>
  import LocalGameControl from "./lib/LocalGameControl.svelte";
  import GameSelector from "./lib/GameSelector.svelte";
  import { gameSettings } from "./lib/stores";
  import BotGameControl from "./lib/BotGameControl.svelte";
  import MultiplayerControl from "./lib/MultiplayerControl.svelte";
  import { fade } from "svelte/transition";

  function handleError(e) {
    console.error(e.detail.message);
    alert("Error: Page will be refreshed");
    location.reload();
  }
  function returnToMenu() {
    gameSettings.set({ mode: 0, modeSettings: {} });
  }
</script>

<head>
  <title>Four In a Row</title>
</head>
<main>
  <h1>Four In a Row</h1>
  {#if $gameSettings.mode === 0}
    <div transition:fade={{ delay: 300, duration: 600 }}>
      <GameSelector />
    </div>
  {:else if $gameSettings.mode === 1}
    <div transition:fade={{ delay: 300, duration: 600 }}>
      <BotGameControl
        on:error={handleError}
        on:menuClicked={returnToMenu}
        botDiff={$gameSettings.modeSettings.botDiff}
        playerIsFirst={$gameSettings.modeSettings.playerIsFirst}
      />
    </div>
  {:else if $gameSettings.mode === 2}
    <div transition:fade={{ delay: 300, duration: 600 }}>
      <LocalGameControl on:error={handleError} on:menuClicked={returnToMenu} />
    </div>
  {:else if $gameSettings.mode === 3}
    <div transition:fade={{ delay: 300, duration: 600 }}>
      <MultiplayerControl
        on:menuClicked={returnToMenu}
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
