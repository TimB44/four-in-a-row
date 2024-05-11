<script>
  import LocalGameControl from "./lib/LocalGameControl.svelte";
  import GameSelector from "./lib/GameSelector.svelte";
  import { gameSettings } from "./lib/stores";
  import BotGameControl from "./lib/BotGameControl.svelte";
  import MultiplayerControl from "./lib/MultiplayerControl.svelte";
  import { fade } from "svelte/transition";
  import Header from "./lib/Header.svelte";
  import Footer from "./lib/Footer.svelte";
  // import { crossfade } from "svelte/transition";

  function handleError(e) {
    console.error(e.detail.message);
    alert("Error: Page will be refreshed");
    location.reload();
  }
  function returnToMenu() {
    gameSettings.set({ mode: 0, modeSettings: {} });
  }
</script>

<Header />
<div class="boardLocation">
  {#if $gameSettings.mode === 0}
    <div class="item" transition:fade={{ delay: 100, duration: 300 }}>
      <GameSelector />
    </div>
  {:else if $gameSettings.mode === 1}
    <div class="item" transition:fade={{ delay: 150, duration: 300 }}>
      <BotGameControl
        on:error={handleError}
        on:menuClicked={returnToMenu}
        botDiff={$gameSettings.modeSettings.botDiff}
        playerIsFirst={$gameSettings.modeSettings.playerIsFirst}
      />
    </div>
  {:else if $gameSettings.mode === 2}
    <div class="item" transition:fade={{ delay: 150, duration: 300 }}>
      <LocalGameControl on:error={handleError} on:menuClicked={returnToMenu} />
    </div>
  {:else if $gameSettings.mode === 3}
    <div class="item" transition:fade={{ delay: 150, duration: 300 }}>
      <MultiplayerControl
        on:menuClicked={returnToMenu}
        on:gameEnd(gameEnd)
        playerIsFirst={$gameSettings.modeSettings["playerIsFirst"]}
        gameId={$gameSettings.modeSettings["id"]}
      />
    </div>
  {/if}
</div>

<Footer />

<style>
  div.item{
    grid-row: 1;
    grid-column: 1;
  }

  div.boardLocation {
    display: grid;
  }
</style>
