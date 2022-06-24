
use super::Board;

pub struct Heuristic {
    start_board: Board,
    gweight : i32,
    hweight : i32
}

impl Heuristic {
    pub fn new(start_board: Board) -> Heuristic {
        Heuristic {
            start_board: start_board,
            gweight: 11,
            hweight: 3
        }
    }

    pub fn calculate_move_cost(&self, current_board: &Board) -> i32 {
        let hval = calculate_manhattan_dist(&current_board);
        let gval = calculate_g_val(&self.start_board, &current_board);

        self.gweight * gval + self.hweight * hval
    }
}

/*
fn calculate_simple_move_cost(current_board: &Board) -> i32 {
    //current_board.cost + calculate_manhattan_dist(current_board)
    calculate_manhattan_dist(current_board)
}
*/

fn calculate_g_val(start_board: &Board, current_board: & Board) -> i32 {
    let mut distance = 0;
    let mut index    = 0;
    let size = start_board.size as i32;
    for tile in &current_board.state {
        let start_index = start_board.get_index_of(tile) as i32;
        let start_row   = start_index / size;
        let start_col   = start_index % size;
        let cur_row     = index / size;
        let cur_col     = index % size;

        distance += i32::abs(start_row - cur_row) + i32::abs(start_col - cur_col);
        index += 1;
    }
    distance
}

fn calculate_manhattan_dist(board: &Board) -> i32 {
    let mut distance = 0;
    let mut index    = 0;
    let last_cell    = 15;

    for tile in &board.state {
        if *tile != 0 {
            distance += calculate_manhattan_dist_tile(index, *tile-1, board.size as i32);
        }
        else {
            distance += calculate_manhattan_dist_tile(index, last_cell, board.size as i32);
        }
        index += 1;
    }
    distance
}

fn calculate_manhattan_dist_tile(index: i32, value: u8, size: i32) -> i32 {
    let current_row = index / size;
    let current_col = index % size;
    let goal_row    = value as i32 / size;
    let goal_col    = value as i32 % size;

    i32::abs(goal_row - current_row) + i32::abs(goal_col - current_col)
}
