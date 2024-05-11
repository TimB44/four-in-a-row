<script>
  import { createEventDispatcher } from "svelte";

  export let gameText;

  let dispatch = createEventDispatcher();

  function replayEvent() {
    dispatch("replayClicked");
  }

  function menuEvent() {
    dispatch("menuClicked");
  }
</script>

<div class="container">
  <h2>{gameText}</h2>
  <button on:click={replayEvent}>Play Again</button>
  <button on:click={menuEvent}>Back To Menu</button>
</div>

<style>
  div.container {
    display: flex;
    flex-direction: column;
    padding: 10px;
    opacity: 0.9;
    width: 350px;
    height: 200px;
    justify-content: space-around;
    align-items: center;
    background-color: #1b1b1b;
    border: 2px solid #d9d9d9;
    border-radius: 30px;
  }
</style>
