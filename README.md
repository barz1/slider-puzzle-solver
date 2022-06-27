# Slider Puzzle Solver
Solver for the sliding puzzle game.

## Repository Overview

 - c-solver - Old solver from college, still works but needs some TLC
 - rust-solver - Core rust solver crate. This crate includes a library 'slider_solver_lib' which contains the core data structures and solver algorithm. It also contains a command line executable 'slider_solver' which can be used to solve 4x4 puzzles
 - rust-wasm-bindings - WASM bindings around the 'slider_solver_lib' mentioned above. Exposes a 'solve' function that can be called from javascript to solve a given puzzle.
 - react-client - React based app to play the game and test the solver, still a work in progress

## Building from source

There is still a decent amount of cleanup to be done, but these instructions capture the current build process.

It assumes you have the following installed:
 - rust
 - wasm-pack
 - node
 - npm or yarn

  1. Build the wasm bindings. NOTE - It currently references the 'rust-solver' lib as a local dependency so that will be built as part of this step
  ```
  cd rust-wasm-bindings
  wasm-pack build
  ```
  2. Install dependencies for react-client. NOTE - The package.json file contains a local 'file' dependency on the rust-wasm-bindings library 'slider-solver-wasm'
  ```
  cd ../react-client/
  yarn install
  ```
  3. Run react-client in dev mode
  ```
  yarn start
  ```
  4. Run react-client in prod mode (assumes npm module 'serve' is installed)
  ```
  yarn build
  serve -s build

  ```

