<script>
    import { createEventDispatcher } from "svelte";

    export let gameText;

    let dispatch = createEventDispatcher();

    function replayEvent(){
        dispatch("replayClicked")
    }

    function menuEvent(){
        dispatch("menuClicked")
    }
</script>
<div id="container">
    <h2>{gameText}</h2>
<button on:click={replayEvent}>Play Again</button>
<button on:click={menuEvent}>Back To Menu</button>
</div>
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
</style>

