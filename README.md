# Join 4

Created By: Tim Blamires
Website URL: [Join 4](https://join4.app)

## Summary

This project is build with 3 main parts.

- game-lib
- game-server
- game-frontend

The game-lib is a Rust library containing the needed types and function to allow for AI players and multiplayer support.

The game-server is a Rust Axum server which exposes a REST api allowing for both AI players and multiplayer games.

The game-frontend is Svelte project which has the UI and the logic to interact with the server.

A Makefile is used to build the project, which compiles the frontend code and copies it into the server. The server is then built using cargo.

A Dockerfile is also used to Dockerize the application, which allows it to be easily ran in the cloud.
