<script>
  import { onMount } from "svelte";
  import { gameSettings } from "./stores";

  let modeSettings = {};
  let buttons = [null, null, null];
  modeSettings = { playerIsFirst: true, botDiff: "easy" };
  let mode = 1;
  let invited = false;

  // Extract the URL parameters if they exist
  let params = new URLSearchParams(document.location.search);
  let first = params.get("fir");
  let id = parseInt(params.get("id"), 10);
  let link = "";
  let main;

  //Clear the parameters after the page loads
  history.pushState(null, "", location.href.split("?")[0]);

  // Auto load game ID if in the URL
  onMount(() => {
    if (!isNaN(id)) {
      invited = true;
      main.className += " background";
      let playerIsFirst = first === "1";
      modeSettings = { id, playerIsFirst };
      mode = 3;
      highlightBtn(2);
      link = getOpponentURL();
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

  function getOpponentURL() {
    let url = window.location.href;
    let urlParams = `?id=${modeSettings["id"]}&fir=${modeSettings["playerIsFirst"] === true ? 0 : 1}`;
    return url + urlParams;
  }

  function submitClicked() {
    $gameSettings = { mode, modeSettings };
  }
  function highlightBtn(i) {
    buttons[0].className = buttons[0].className.replace(" active", "");
    buttons[1].className = buttons[1].className.replace(" active", "");
    buttons[2].className = buttons[2].className.replace(" active", "");

    buttons[i].className += " active";
  }
</script>

<div class="slot">
  {#if invited}
    <div class="container">
      <div class="modeButtons horizontal" style="justify-content: center;">
        <h3 style="margin: 6px;">Game Invite</h3>
      </div>
      <div class="horizontal">
        <button
          class="margin"
          on:click={() => {
            invited = false;
            submitClicked();
          }}>Play</button
        >
        <button
          class="margin"
          on:click={() => {
            modeSettings = { playerIsFirst: true, botDiff: "easy" };
            mode = 1;
            invited = false;
          }}>Back</button
        >
      </div>

      <div></div>
    </div>
  {:else}
    <div class="container" bind:this={main}>
      <div class="modeButtons">
        <button
          bind:this={buttons[0]}
          class="modeButton active"
          id="left"
          on:click={() => {
            mode = 1;
            modeSettings = { playerIsFirst: true, botDiff: "easy" };
            highlightBtn(0);
          }}>Single Player</button
        >
        <button
          bind:this={buttons[1]}
          class="modeButton"
          on:click={() => {
            mode = 2;
            modeSettings = {};
            highlightBtn(1);
          }}>Local Multiplayer</button
        >
        <button
          bind:this={buttons[2]}
          class="modeButton"
          id="right"
          on:click={async () => {
            mode = 3;

            modeSettings = { id, playerIsFirst: true };
            highlightBtn(2);

            modeSettings["id"] = await getGameId();
            link = getOpponentURL();
          }}>Online Multiplayer</button
        >
      </div>

      {#if mode === 1}
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
          <span>(Red Goes first)</span>

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
      {:else if mode === 2}
      <div>
        <h3>Click Play to Begin</h3>
        <h4>Red Goes First</h4>
      </div>
        
      {:else if mode === 3}
        <div class="settingsContainer">
          <h3>Choose Your Color</h3>
          <label>
            <input
              checked={modeSettings["playerIsFirst"] === true}
              name="color"
              type="radio"
              bind:group={modeSettings["playerIsFirst"]}
              value={true}
              on:click={async () => {
                modeSettings["playerIsFirst"] = true;
                link = getOpponentURL();
              }}
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
              on:click={async () => {
                modeSettings["playerIsFirst"] = false;
                link = getOpponentURL();
              }}
            />
            Blue
          </label>
          <span>(Red Goes first)</span>
          <h3>Link for Opponent</h3>
          <div class="horizontal">
            <div
              class="margin textbox"
              contenteditable="false"
              bind:innerText={link}
            ></div>
            <!-- <p contenteditable="false" bind:innerText={link}></p> -->
            <button
              class="margin"
              on:click={async () => {
                modeSettings["id"] = await getGameId();
                link = getOpponentURL();
              }}>New Link</button
            >
            <button
              class="margin"
              on:click={() => navigator.clipboard.writeText(getOpponentURL())}
              >Copy</button
            >
          </div>
        </div>
      {/if}
      <button class="margin" on:click={submitClicked}>Play</button>
    </div>
  {/if}
</div>

<style>
  div.horizontal {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: stretch;
  }

  .textbox {
    background-color: #313131;
    border-radius: 5px;
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 7px;
  }

  .margin {
    margin: 5px;
  }
  div.container {
    display: flex;
    flex-direction: column;
    opacity: 0.9;
    width: 500px;
    height: 300px;
    justify-content: space-between;
    align-items: center;
    background-color: #1b1b1b;
    border: 3px solid #d9d9d9;
    border-radius: 30px;
    grid-row: 1;
    grid-column: 1;
  }
  button.modeButton {
    flex-grow: 1;
    border-radius: 3px;
    background-color: #1b1b1b;
  }
  button.active {
    background-color: #313131;
  }
  #left {
    border-top-left-radius: 26px;
  }
  #right {
    border-top-right-radius: 26px;
  }
  div.modeButtons {
    border-bottom: 3px solid #d9d9d9;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    width: 100%;
  }

  div.slot {
    display: grid;
    justify-items: center;
    align-items: center;
  }
  span {
    opacity: .7;
  }
</style>
