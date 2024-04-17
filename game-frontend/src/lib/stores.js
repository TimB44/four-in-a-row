import { writable } from "svelte/store";

export const gameSettings = writable({
  mode: 1, //1 is single, 2 is local, 3 is online
  modeSettings: {},
});
