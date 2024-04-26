<script>
  import { createEventDispatcher } from "svelte";
  import { gameIsOver, top } from "./game-lib";
  import Board from "./Board.svelte";
  import GameButtons from "./GameButtons.svelte";
  import { onMount } from "svelte";

  export let playerIsFirst = true;
  export let gameId = -1;
  const dispatch = createEventDispatcher();

  let disabled = Array(7).fill(false);
  let board = Array.from({ length: 6 }, () => Array(7).fill(0));
  let turns = 0;
  let buttons;
  let visualBoard;
  console.log("in multiplayer game control.");
  console.log(`player is first: ${playerIsFirst}`);

  // If no gameID given then fetch one form the server and give player link

  onMount(() => {
    if (playerIsFirst) {
      buttons.enable();
    } else {
      buttons.disable();
      getOpponentMove();
    }
    
  });

  /**
   * Ends the game and triggers and event to notify its parent
   * @param {String} winner
   */
  function endGame(winner) {
    dispatch("gameEnd", {
      message: winner,
      error: false,
    });
  }
  function throwError() {
    dispatch("gameEnd", { error: true });
  }

  function playMove(col) {
    buttons.disable();

    let row = top(col, board);

    if (row === 5) disabled[col] = true;

    //first player is 1, second -1
    let player = playerIsFirst ? 1 : -1;

    board[row][col] = player;
    visualBoard.placePiece(row, col, player === 1 ? "red" : "blue");
    turns++;
    
    sendNewMove(col);
    let winner = gameIsOver(board);

    if (winner !== 0) {
      endGame(`${winner == 1 ? "Red" : "Blue"} Wins!`);
      return;
    }

    if (turns == 42) {
      endGame("Draw");
      return;
    }
    getOpponentMove();
  }

  async function sendNewMove(col) {
    let resp = await fetch("/multiplayer/make_move", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        game_move: col,
        is_first: playerIsFirst,
        game_id: gameId,
      }),
    });

    if (!resp.ok) {
      console.log(await resp.blob());
      throwError();
    }
    getOpponentMove();
  }
  async function getOpponentMove() {
    let promise = fetch("/multiplayer/get_board", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        game_id: gameId,
        // difficulty: botDiff,
        current_board: board,
      }),
    });

    let resp = await promise;

    if (!resp.ok) {
      throwError();
    }

    let json = await resp.json();

    let newBoard = json["board"];
    playNewMoves(newBoard);
    if (
      (turns % 2 === 0 && playerIsFirst) ||
      (turns % 2 === 1 && !playerIsFirst)
    ) {
      for (let i = 0; i < 7; i++) {
        buttons.setCol(i, disabled[i]);
      }
    } else {
      console.log("BAD"); //TODO REMOVE
      getOpponentMove();
    }
  }

  function playNewMoves(newBoard) {
    for (let col = 0; col < 7; col++) {
      for (let row = 0; row < 6; row++) {
        if (newBoard[row][col] !== board[row][col]) {
          if (board[row][col] !== 0) {
            throwError();
            return;
          }
          board[row][col] = newBoard[row][col];
          visualBoard.placePiece(
            row,
            col,
            board[row][col] === 1 ? "red" : "blue"
          );
          turns++;
        }
      }
    }
  }

  //Starts a new game session
  export function start() {
    if (!playerIsFirst) {
      buttons.disable();
    } else {
      buttons.enable();
    }
  }

  // Clears the board to the state it is when first created
  export function clear() {
    disabled.fill(false);
    board.forEach((row) => row.fill(0));
    turns = 0;
    visualBoard.clear();
    buttons.disable();
  }

  // Starts a new game with some state shared between sessions
  // In this game setting it is the same as start
  export function restart() {
    clear();
    start();
  }
</script>

<div>
  <Board bind:this={visualBoard} />
  <GameButtons
    bind:this={buttons}
    on:buttonClick={(e) => {
      playMove(e.detail.col);
    }}
  ></GameButtons>
</div>

<style>
</style>
