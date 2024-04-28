<script>
  import { tweened } from "svelte/motion";
  import { cubicOut } from "svelte/easing";

  export let row;
  export let col;
  export let color;

  let x = 50 + 100 * col;
  let y = tweened(50, {
    delay: 50,
    easing: cubicOut,
    duration: (to, from) => Math.sqrt(Math.abs(to - from) * 400),
  });

  y.set(150 + 100 * (5 - row));
</script>

<circle cx={x} cy={$y} r="45" fill={color === "red" ? "#E53734" : "#04BEBE"} />
