<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";
  import { gameSettings } from "./stores";

  let dispatch = createEventDispatcher();

  // import { scale } from "svelte/transition";
  let settings = {};
  let mode = 1;

  // $: console.log(settings);
  $: gameSettings.set({ mode, modeSettings: settings });

  //TODO change
  let firstBtn;

  // Extract the URL parameters if they exist
  let params = new URLSearchParams(document.location.search);
  let first = params.get("fir");
  let id = parseInt(params.get("id"), 10);

  //Clear the parameters after the page loads
  history.pushState(null, "", location.href.split("?")[0]);


  onMount(() => {
    if (!isNaN(id)) {
      let isFir = first === "1";

      settings = { id, playerIsFirst: isFir };
      mode = 3;
    } else {
      settings = { playerIsFirst: true, botDiff: "easy" };
    }
  });

  /**
   * Returns a valid game ID
   */
  async function getGameId() {
    let promise = fetch("/multiplayer/create_game", {
      method: "POST",
      headers: {
        "Content-Type": "0",
      },
    });
    let resp = await promise;
    let json = await resp.json();
    return json["id"];
  }

  function copyUrlToClipboard(
    event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }
  ) {
    let url = window.location.href;
    let urlParams = `?id=${settings["id"]}&fir=${settings["playerIsFirst"] === true ? 0 : 1}`;
    navigator.clipboard.writeText(url + urlParams);
  }
</script>

<div id="container">
  <div class="modeButtons">
    <button
      bind:this={firstBtn}
      on:click={() => {
        mode = 1;
        settings = { playerIsFirst: true, botDiff: "easy" };
      }}>Single Player</button
    >
    <button
      on:click={() => {
        mode = 2;
        settings = {};
      }}>Local Multiplayer</button
    >
    <button
      on:click={() => {
        mode = 3;
        settings = { id: "None", playerIsFirst: true };
      }}>Online Multiplayer</button
    >
  </div>

  {#if mode == 1}
    <div class="settingsContainer">
      <h3>Choose Your Color</h3>
      <label>
        <input
          checked={settings["playerIsFirst"] === true}
          name="color"
          type="radio"
          bind:group={settings["playerIsFirst"]}
          value={true}
        />
        Red
      </label>
      <label>
        <input
          checked={settings["playerIsFirst"] === false}
          name="color"
          type="radio"
          bind:group={settings["playerIsFirst"]}
          value={false}
        />
        Blue
      </label>

      <h3>AI Difficulty</h3>

      <label>
        <input
          checked={settings["botDiff"] === "easy"}
          name="AiDifficulty"
          type="radio"
          bind:group={settings.botDiff}
          value={"easy"}
        />
        Easy
      </label>
      <label>
        <input
          checked={settings["botDiff"] === "medium"}
          name="AiDifficulty"
          type="radio"
          bind:group={settings.botDiff}
          value={"medium"}
        />
        Medium
      </label>
      <label>
        <input
          checked={settings["botDiff"] === "hard"}
          name="AiDifficulty"
          type="radio"
          bind:group={settings.botDiff}
          value={"hard"}
        />
        Hard
      </label>
    </div>
  {:else if mode == 3}
    <div class="settingsContainer">
      <h3>Choose Your Color</h3>
      <label>
        <input
          checked={settings["playerIsFirst"] === true}
          name="color"
          type="radio"
          bind:group={settings["playerIsFirst"]}
          value={true}
        />
        Red
      </label>
      <label>
        <input
          checked={settings["playerIsFirst"] === false}
          name="color"
          type="radio"
          bind:group={settings["playerIsFirst"]}
          value={false}
        />
        Blue
      </label>
      <h3>Game ID</h3>
      <p contenteditable="false" bind:innerText={settings["id"]}></p>
      <button
        on:click={() => {
          getGameId().then((id) => (settings["id"] = id));
        }}>Get ID</button
      >
      <button on:click={copyUrlToClipboard}>Copy URL for opponent</button>
    </div>
  {/if}
  <button on:click={() => dispatch("gameStart")}>play</button>
</div>

<!-- <h2>Game Options</h2>
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
    <todo add more ai -->
<!-- </div>
</div> -->

<style>
  #container {
    width: 700px;
    height: 500px;
    border: 2px solid #d9d9d9;
    border-radius: 30px;
    display: flex;
    flex-direction: column;

    align-items: center;
    background-color: #1b1b1b;
  }
  div.modeButtons {
    border-bottom: 3px solid #d9d9d9;
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    width: 100%;
  }
</style>
