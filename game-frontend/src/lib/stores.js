import { writable } from "svelte/store";

export const gameSettings = writable({
  mode: 0, //0 is menu selector, 1 is single, 2 is local, 3 is online
  modeSettings: {},
});
