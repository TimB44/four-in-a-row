<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
  let buttons = [];

  export function disable() {
    buttons.forEach((b) => b.disabled = true);
  }

  export function enable() {
    buttons.forEach((b) => b.disabled = false);
  }
  /**
   * disables/enables the button at the specified column
   *
   * @param {number} col
   * @param {boolean} val
   */
  export function setCol(col, val) {
    if (col < 0 || col > 6) return;

    buttons[col].disabled = val;
  }
  
  /**
   * Fires an event with the column of the button at given.
   * @param {number} col
   */
  function fireClickEvent(col) {
    if (col < 0 || col > 6) return;

    dispatch("buttonClick", {col: col});

  }
</script>

<div>
    {#each Array(7) as _, i (i)}
      <button bind:this={buttons[i]} on:click={() => fireClickEvent(i)}></button>
    {/each}
</div>

<style>
    button {
        background-color: #d9d9d9;
    }
</style>
