#![feature(map_first_last)]
mod solver;

pub use solver::{Board, Move, bidirectional_solver};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn board_from_js(state: js_sys::Uint8Array, p_move: solver::Move, cost: i32) -> solver::Board {
    solver::Board::new(
        state.to_vec(),
        p_move,
        cost
    )
}

