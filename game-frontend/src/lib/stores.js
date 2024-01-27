import { writable } from "svelte/store";

export const gameSettings = writable({
    mode: 1, //1 is local, 2 is bot, 3 is online TODO
    botSettings: {
        botDiff: 'easy',
        playerIsFirst: true,
    },
})