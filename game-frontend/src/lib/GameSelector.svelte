<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";
  import { gameSettings } from "./stores";

  // import { scale } from "svelte/transition";
  let modeSettings = {};
  modeSettings = { playerIsFirst: true, botDiff: "easy" };
  let mode = 1;

  // Extract the URL parameters if they exist
  let params = new URLSearchParams(document.location.search);
  let first = params.get("fir");
  let id = parseInt(params.get("id"), 10);

  //Clear the parameters after the page loads
  history.pushState(null, "", location.href.split("?")[0]);
  console.log("here1");
  console.log(id);



  // Auto load game ID if in the URL
  onMount(() => {
    if (!isNaN(id)) {
      console.log("here");
      
      let isFir = first === "1";

      modeSettings = { id, playerIsFirst: isFir };
      mode = 3;
    } else {
      modeSettings = { playerIsFirst: true, botDiff: "easy" };
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

  function copyUrlToClipboard() {
    let url = window.location.href;
    let urlParams = `?id=${modeSettings["id"]}&fir=${modeSettings["playerIsFirst"] === true ? 0 : 1}`;
    navigator.clipboard.writeText(url + urlParams);
  }

  function submitClicked(){
    $gameSettings = {mode, modeSettings};
  }
</script>

<div id="container">
  <div class="modeButtons">
    <button
      on:click={() => {
        mode = 1;
        modeSettings = { playerIsFirst: true, botDiff: "easy" };
      }}>Single Player</button
    >
    <button
      on:click={() => {
        mode = 2;
        modeSettings = {};
      }}>Local Multiplayer</button
    >
    <button
      on:click={() => {
        mode = 3;
        modeSettings = { id: "None", playerIsFirst: true };
      }}>Online Multiplayer</button
    >
  </div>

  {#if mode == 1}
    <div class="settingsContainer">
      <h3>Choose Your Color</h3>
      <label>
        <input
          checked={modeSettings["playerIsFirst"] === true}
          name="color"
          type="radio"
          bind:group={modeSettings["playerIsFirst"]}
          value={true}
        />
        Red
      </label>
      <label>
        <input
          checked={modeSettings["playerIsFirst"] === false}
          name="color"
          type="radio"
          bind:group={modeSettings["playerIsFirst"]}
          value={false}
        />
        Blue
      </label>

      <h3>AI Difficulty</h3>

      <label>
        <input
          checked={modeSettings["botDiff"] === "easy"}
          name="AiDifficulty"
          type="radio"
          bind:group={modeSettings["botDiff"]}
          value={"easy"}
        />
        Easy
      </label>
      <label>
        <input
          checked={modeSettings["botDiff"] === "medium"}
          name="AiDifficulty"
          type="radio"
          bind:group={modeSettings["botDiff"]}
          value={"medium"}
        />
        Medium
      </label>
      <label>
        <input
          checked={modeSettings["botDiff"] === "hard"}
          name="AiDifficulty"
          type="radio"
          bind:group={modeSettings["botDiff"]}
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
          checked={modeSettings["playerIsFirst"] === true}
          name="color"
          type="radio"
          bind:group={modeSettings["playerIsFirst"]}
          value={true}
        />
        Red
      </label>
      <label>
        <input
          checked={modeSettings["playerIsFirst"] === false}
          name="color"
          type="radio"
          bind:group={modeSettings["playerIsFirst"]}
          value={false}
        />
        Blue
      </label>
      <h3>Game ID</h3>
      <p contenteditable="false" bind:innerText={modeSettings["id"]}></p>
      <button
        on:click={() => {
          getGameId().then((id) => (modeSettings["id"] = id));
        }}>Get ID</button
      >
      <button on:click={copyUrlToClipboard}>Copy URL for opponent</button>
      <span>{`?id=${modeSettings["id"]}&fir=${modeSettings["playerIsFirst"] === true ? 0 : 1}`}</span>
    </div>
  {/if}
  <button on:click={submitClicked}>Play</button>
</div>
<style>
  #container {
    width: 700px;
    height: 400px;
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
