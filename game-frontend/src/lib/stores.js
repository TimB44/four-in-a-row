import { writable } from "svelte/store";

export const gameSettings = writable({
    firstPlayerIsRed: true,
    isOnePlayer: true,
    aiDifficulty: 'easy',
})