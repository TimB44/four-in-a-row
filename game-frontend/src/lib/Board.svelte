<script>
    import Piece from "./Piece.svelte"
    

    let pieces = []
    export function placePiece(row, col, color) {
        pieces = [...pieces, {row, col, color}];
    }
    export function clear() {
        pieces = [];
    }
</script>

<!-- some svg code adapted form https://rossta.net/blog/connect-four-with-svg-pattern-masking.html -->
<svg viewBox="0 0 700 700" xmlns="http://www.w3.org/2000/svg">

    <defs>
      <pattern id="cell-pattern" patternUnits="userSpaceOnUse" width="100" height="100">
        <circle  cx="50" cy="50" r="45" fill="black"></circle>
      </pattern>
      <mask id="cell-mask">
        <rect width="700" height="700" fill="white"></rect>
        <rect width="700" height="700" fill="url(#cell-pattern)"></rect>
      </mask>
    </defs>
    {#each pieces as { row, col, color}, i (i)}
        <Piece row = {row} col = {col} color = {color}></Piece>
  {/each}

    <rect x="0" y="100" height="600" width="700" fill="#303030" mask="url(#cell-mask)"></rect>

</svg>

<style>
    svg {
        width: 700px;
        height: 700px;
    }
</style>