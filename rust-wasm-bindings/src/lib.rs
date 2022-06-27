use slider_solver_lib::{Board, Move, bidirectional_solver};
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn solve(state: js_sys::Uint8Array) -> js_sys::Uint8Array {
    utils::set_panic_hook();


    if state.length() == 16 {
        let mut native_state : [u8; 16] = [0; 16];
        for ii in 0..16 as i32{
            native_state[ii as usize] = state.at(ii).unwrap();
        }

        let start = Board {
            size: 4,
            state: native_state,
            m_list: Vec::new(),
            cost: 0,
            score: 0
        };

        let end : [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let goal = Board {
            size: 4,
            state: end,
            m_list: Vec::new(),
            cost: 0,
            score: 0
        };

        let solution = bidirectional_solver(&start, &goal);

        let mut native_solution : Vec<u8> = Vec::new();
        for the_move in solution {
            match the_move {
                Move::Up    => native_solution.push(0),
                Move::Down  => native_solution.push(1),
                Move::Left  => native_solution.push(2),
                Move::Right => native_solution.push(3),
                Move::None  => ()
            }
        }

        js_sys::Uint8Array::from(&native_solution[..])

    }
    else {
        js_sys::Uint8Array::new_with_length(1)
    }
}

#[wasm_bindgen]
pub fn generate(size: usize) -> js_sys::Uint8Array {
    let board = Board::generate_board(size);

    js_sys::Uint8Array::from(&board.state[..])
}
