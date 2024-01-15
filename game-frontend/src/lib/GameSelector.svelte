<script>
  import { createEventDispatcher } from "svelte";
  let dispatch = createEventDispatcher();

  import { gameSettings } from "./stores";

  let isOnePlayer = true;
  let firstPlayerIsRed = true;
  let aiDifficulty = isOnePlayer ? "easy" : "n/a";
  $: gameSettings.set({ firstPlayerIsRed, aiDifficulty, isOnePlayer });
</script>

<h2>Game Options</h2>
<div class="container">
  <div>
    <h3>Player 1 Color</h3>
    <label>
      <input
        checked={true}
        name="color"
        type="radio"
        bind:group={firstPlayerIsRed}
        value={true}
      />
      Red
    </label>

    <label>
      <input
        name="color"
        type="radio"
        bind:group={firstPlayerIsRed}
        value={false}
      />
      Blue
    </label>
  </div>

  <div>
    <h3>Number Of Players</h3>
    <label>
      <input
        checked={true}
        name="playerNum"
        type="radio"
        bind:group={isOnePlayer}
        value={true}
      />
      One Player
    </label>

    <label>
      <input
        name="playerNum"
        type="radio"
        bind:group={isOnePlayer}
        value={false}
      />
      Two Player
    </label>
  </div>

  <div>
    <h3>AI Difficulty</h3>

    <label>
      <input
        checked={true}
        name="AiDifficulty"
        type="radio"
        bind:group={aiDifficulty}
        value={"easy"}
        disabled={!isOnePlayer}
      />
      Easy
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
