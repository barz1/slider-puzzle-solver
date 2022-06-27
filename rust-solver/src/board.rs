use std::hash::{Hash, Hasher};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
    None
}

pub struct Board {
    // The size of row/column for the board
    pub size: usize,

    // The current layout of the board
    //state: [u8; 9],
    pub state: [u8; 16],

    // The parent board this board came from
    pub m_list: Vec<Move>,

    // Count of how many moves from start board
    pub cost: i32,

    // Heuristics value for this board
    pub score: i32
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board{
            size: self.size,
            state: self.state.clone(),
            m_list: self.m_list.clone(),
            cost:  self.cost,
            score: self.score
        }
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Eq for Board {}

// Public functions
impl Board {
    pub fn expand(&self, heuristics: &super::Heuristic) -> Vec<Board> {
        let mut moves = Vec::new();

        let move_up    = Board::move_up(&self);
        if let Some(mut board) = move_up {
            board.score = heuristics.calculate_move_cost(&board);
            moves.push(board);
        }

        let move_down  = Board::move_down(&self);
        if let Some(mut board) = move_down {
            board.score = heuristics.calculate_move_cost(&board);
            moves.push(board);
        }

        let move_left  = Board::move_left(&self);
        if let Some(mut board) = move_left {
            board.score = heuristics.calculate_move_cost(&board);
            moves.push(board);
        }

        let move_right = Board::move_right(&self);
        if let Some(mut board) = move_right {
            board.score = heuristics.calculate_move_cost(&board);
            moves.push(board);
        }

        moves
    }

    pub fn is_solvable(&self) -> bool {
        let mut inversions = 0;
        let mut zero_row = 0;
        let row_size = self.size;

        for index in 0..self.state.len() {

            let current = self.state[index];
            if current == 0 {
                zero_row = index / row_size;
                continue;
            }

            for nindex in index+1..self.state.len() {
                if self.state[nindex] != 0 && current > self.state[nindex] {
                    inversions += 1;
                }
            }
        }

        // For odd size boards (ex. 3x3) number of inversions must be even
        if self.state.len() % 2 != 0 {
            return inversions % 2 == 0;
        }

        // For even size boards (ex. 4x4) number of inversions + row of blank square must be odd
        (inversions + zero_row) % 2 != 0
    }

    pub fn get_index_of(&self, input: &u8) -> usize {
        let mut index: usize = 0;
        for val in &self.state {
            if *val == *input {
                break;
            }
            index += 1;
        }

        index
    }

    pub fn generate_board(size: usize) -> Board {
        let mut solvable = false;
        let mut values : Vec<u8> = (0..size*size).map(|x| x as u8).collect();

        while !solvable {
            values.shuffle(&mut thread_rng());

            let state = Board::vec_to_array(&values);
            solvable = Board::is_solvable_arr(state, size);
        }

        Board {
            size,
            state: Board::vec_to_array(&values),
            m_list: Vec::new(),
            cost : 0,
            score: 0
        }
    }

    pub fn print(&self) {
        println!("Printing board");
        for row in 0..self.size {
            for col in 0..self.size {
                print!("{} ", self.state[row*self.size+col]);
            }
            println!();
        }
        //println!("0 {} {} 0\n", self.gval, self.hval);
        //println!();
    }

    pub fn print_flat(&self) {
        for row in 0..self.size {
            for col in 0..self.size {
                print!("{} ", self.state[row*self.size+col]);
            }
        }
    }
}

// Private functions
impl Board {
    fn move_left(parent_board: &Board) -> Option<Board> {
        let index = parent_board.get_index_of(&0);

        // Cant move left from first column
        if index % parent_board.size == 0 {
            return None
        }

        let mut new_state = parent_board.state.clone();
        new_state[index]   = new_state[index-1]; 
        new_state[index-1] = 0;

        let mut board = parent_board.clone();
        board.state = new_state;
        board.cost += 1;
        board.m_list.push(Move::Left);
        Some(board)
    }

    fn move_right(parent_board: &Board) -> Option<Board> {
        let index = parent_board.get_index_of(&0);

        // Cant move right from last column
        if index % parent_board.size == parent_board.size - 1 {
            return None
        }

        let mut new_state = parent_board.state.clone();
        new_state[index]   = new_state[index+1]; 
        new_state[index+1] = 0;

        let mut board = parent_board.clone();
        board.state = new_state;
        board.cost += 1;
        board.m_list.push(Move::Right);
        Some(board)
    }

    fn move_up(parent_board: &Board) -> Option<Board> {
        let index = parent_board.get_index_of(&0);

        // Cant move up from first row
        if index < parent_board.size {
            return None
        }

        let mut new_state = parent_board.state.clone();
        new_state[index]   = new_state[index-parent_board.size]; 
        new_state[index-parent_board.size] = 0;

        let mut board = parent_board.clone();
        board.state = new_state;
        board.cost += 1;
        board.m_list.push(Move::Up);
        Some(board)
    }

    fn move_down(parent_board: &Board) -> Option<Board> {
        let index = parent_board.get_index_of(&0);

        // Cant move down up from last row
        if index / parent_board.size == parent_board.size - 1 {
            return None
        }

        let mut new_state = parent_board.state.clone();
        new_state[index]   = new_state[index+parent_board.size]; 
        new_state[index+parent_board.size] = 0;

        let mut board = parent_board.clone();
        board.state = new_state;
        board.cost += 1;
        board.m_list.push(Move::Down);
        Some(board)
    }

    fn is_solvable_arr(state: [u8;16], row_size: usize) -> bool {
        let mut inversions = 0;
        let mut zero_row = 0;

        for index in 0..row_size*row_size {

            let current = state[index];
            if current == 0 {
                zero_row = index / row_size;
                continue;
            }

            for nindex in index+1..row_size*row_size {
                if state[nindex] != 0 && current > state[nindex] {
                    inversions += 1;
                }
            }
        }

        // For odd size boards (ex. 3x3) number of inversions must be even
        if row_size % 2 != 0 {
            return inversions % 2 == 0;
        }

        // For even size boards (ex. 4x4) number of inversions + row of blank square must be odd
        (inversions + zero_row) % 2 != 0
    }

    fn vec_to_array(vector : &Vec<u8>) -> [u8;16] {
        let mut array : [u8;16] = [0;16];

        let mut ii : usize = 0;
        for entry in vector {
            array[ii] = *entry;
            ii += 1;
        }
        array
    }

}
