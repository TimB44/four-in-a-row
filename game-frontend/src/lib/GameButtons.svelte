<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
  let buttons = [];

  export function disable() {
    buttons.forEach((b) => (b.disabled = true));
  }

  export function enable() {
    buttons.forEach((b) => (b.disabled = false));
  }
  /**
   * disables/enables the button at the specified column
   *
   * @param {number} col
   * @param {boolean} disable
   */
  export function setCol(col, disable) {
    if (col < 0 || col > 6) return;

    buttons[col].disabled = disable;
  }

  /**
   * Fires an event with the column of the button at given.
   * @param {number} col
   */
  function fireClickEvent(col) {
    if (col < 0 || col > 6) return;

    dispatch("buttonClick", { col: col });
  }
</script>

<div>
  {#each Array(7) as _, i (i)}
    <button bind:this={buttons[i]} on:click={() => fireClickEvent(i)}></button>
  {/each}
</div>

<style>
  div {
    width: max(min(calc(100vh - 300px), min(1200px, 100vw)), 300px);
    height: min(
      calc(max(min(calc(100vh - 300px), min(1200px, 100vw)), 300px) / 7),
      100px
    );
    display: flex;
    flex-direction: row;
    align-items: stretch;
    justify-content: center;
    flex-grow: 1;
  }
  button {
    width: 100%;
    margin: 5px;
  }
</style>
