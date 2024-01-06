<script>
  import { createEventDispatcher } from "svelte";

  export let playerOneIsRed = true;
  export let isAi = false;
  export let difficulty = isAi ? "n/a" : "easy";

  const dispatch = createEventDispatcher();
  let turns = 0;
  let buttons = [];
  let disabled = Array(7).fill(false);
  let board = Array.from({ length: 6 }, () => Array(7).fill(0));

  function gameOver() {
    dispatch("gameover", {
      winner: "Red/Blue TODO change",
    });
  }

  function top(col) {
    for (let i = 0; i < board.length; i++) {
      if (board[i][col] === 0) {
        return i;
      }
    }
    return 6;
  }

  function playMove(col) {
    buttons.forEach((element) => {
      element.disabled = true;
    });
    let row = top(col);

    if (row === 5) disabled[col] = true;

    let piece = turns % 2 === 0 && playerOneIsRed ? "R" : "B";

    board[row][col] = piece;
    turns++;

    if (!isAi) {
      for (let i = 0; i < disabled.length; i++) {
        buttons[i].disabled = disabled[i];
      }
    } else {
      //todo
    }

    board = board;
  }
</script>

{#each Array(6) as _, i (i)}
  <div>{board[5 - i]}</div>
{/each}
<div>
  {#each Array(7) as _, i (i)}
    <button bind:this={buttons[i]} on:click={() => playMove(i)}
    ></button>
  {/each}
</div>
