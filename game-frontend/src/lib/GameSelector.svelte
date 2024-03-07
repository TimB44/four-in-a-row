<script>
  import { createEventDispatcher } from "svelte";
  let dispatch = createEventDispatcher();

  import { gameSettings } from "./stores";

  let mode = $gameSettings.mode;
  let botDiff = $gameSettings.botSettings.botDiff;
  let playerIsFirst = $gameSettings.botSettings.playerIsFirst;
  $: gameSettings.set({ mode: mode, botSettings: { botDiff, playerIsFirst } });
</script>

<h2>Game Options</h2>
<div class="container">
  <div>
    <h3>Player 1 Color</h3>
    <label>
      <input
        checked={playerIsFirst}
        name="color"
        type="radio"
        bind:group={playerIsFirst}
        value={true}
      />
      Red
    </label>

    <label>
      <input
        checked={!playerIsFirst}
        name="color"
        type="radio"
        bind:group={playerIsFirst}
        value={false}
      />
      Blue
    </label>
  </div>

  <div>
    <h3>Number Of Players</h3>
    <label>
      <input
        checked={mode == 2}
        name="playerNum"
        type="radio"
        bind:group={mode}
        value={2}
      />
      One Player
    </label>

    <label>
      <input
        checked={mode == 1}
        name="playerNum"
        type="radio"
        bind:group={mode}
        value={1}
      />
      Two Player
    </label>
  </div>

  <div>
    <h3>AI Difficulty</h3>

    <label>
      <input
        checked={botDiff === "easy"}
        name="AiDifficulty"
        type="radio"
        bind:group={botDiff}
        value={"easy"}
        disabled={mode !== 2}
      />
      Easy
    </label>
    <label>
      <input
        checked={botDiff === "medium"}
        name="AiDifficulty"
        type="radio"
        bind:group={botDiff}
        value={"medium"}
        disabled={mode !== 2}
      />
      Medium
    </label>
    <label>
      <input
        checked={botDiff === "hard"}
        name="AiDifficulty"
        type="radio"
        bind:group={botDiff}
        value={"hard"}
        disabled={mode !== 2}
      />
      Hard
    </label>
    <!-- todo add more ai -->
  </div>
</div>

<button
  on:click={(e) => {
    dispatch("gameStart", {});
  }}>Start</button
>

<style>
  div.container {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }
</style>
