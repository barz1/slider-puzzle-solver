use std::env;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug)]
enum Move {
    Left,
    Right,
    Up,
    Down
}

#[derive(Eq)]
struct HeapEntry {
    board: Board,
    priority: i32,
    order: i32
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.priority, self.order).cmp(&(other.priority, other.order)).reverse()
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        (self.priority, self.order) == (other.priority, other.order)
    }
}

struct Metrics {
    path: Vec<Move>,
    cost_of_path: i32,
    nodes_expanded: i32,
    max_search_depth: i32,
    search_depth: i32,
    running_time: Duration
}

impl Metrics {
    fn display(&self) {
        println!("path_to_goal: {:?}", self.path);
        println!("cost_of_path: {}", self.cost_of_path);
        println!("nodes_expanded: {}", self.nodes_expanded);
        println!("search_depth: {}", self.search_depth);
        println!("max_search_depth: {}", self.max_search_depth);
        println!("running_time: {:?}", self.running_time);
    }
}


struct Board {
    // The size of the board
    size: usize,

    // The current layout of the board
    //state: [u8; 9],
    state: [u8; 16],

    // The parent board this board came from
    m_list: Vec<Move>,

    cost: i32,
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board{
            size: self.size,
            state: self.state.clone(),
            m_list: self.m_list.clone(),
            cost:  self.cost,
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

impl Board {
    fn expand(&self) -> Vec<Board> {
        let mut moves = Vec::new();

        let move_up    = Board::move_up(&self);
        if let Some(board) = move_up {
            moves.push(board);
        }

        let move_down  = Board::move_down(&self);
        if let Some(board) = move_down {
            moves.push(board);
        }

        let move_left  = Board::move_left(&self);
        if let Some(board) = move_left {
            moves.push(board);
        }

        let move_right = Board::move_right(&self);
        if let Some(board) = move_right {
            moves.push(board);
        }

        moves
    }

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

    fn get_index_of(&self, input: &u8) -> usize {
        let mut index: usize = 0;
        for val in &self.state {
            if *val == *input {
                break;
            }
            index += 1;
        }

        index
    }

    fn print(&self) {
        println!("Printing board");
        for row in 0..self.size {
            for col in 0..self.size {
                print!("{} ", self.state[row*self.size+col]);
            }
            println!();
        }
        println!();
    }
}

fn is_solvable(board: &Board) -> bool {
    let mut inversions = 0;
    let mut zero_index = 0;

    for index in 0..board.state.len() {

        let current = board.state[index];
        if current == 0 {
            zero_index = index;
            continue;
        }

        for nindex in index+1..board.state.len() {
            if board.state[nindex] != 0 && current > board.state[nindex] {
                inversions += 1;
            }
        }
    }

    if board.state.len() % 2 != 0 {
        return inversions % 2 == 0;
    }
    inversions + (zero_index / board.state.len()) % 2 != 0
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn test_goal(board : &Board, goal : &Board) -> bool {
    board.state == goal.state
}

fn calculate_move_cost(start_board: &Board, current_board: &Board) -> i32 {
    println!("{}, {}", (calculate_manhattan_dist(current_board)), (calculate_g_val(start_board, current_board)));
    (3 * calculate_manhattan_dist(current_board)) + (11 * calculate_g_val(start_board, current_board))
}

fn calculate_simple_move_cost(current_board: &Board) -> i32 {
    current_board.cost + calculate_manhattan_dist(current_board)
}

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

fn perform_move(board: Board, frontier: &mut BinaryHeap<HeapEntry>, explored: &mut HashSet<Board>, entry_count: &mut HashMap<i32, i32>) {
    let children = board.expand();

    explored.insert(board);

    for child in children {

        if !explored.contains(&child) {
            let cost = calculate_simple_move_cost(&child);

            *entry_count.entry(cost).or_insert(1) += 1;
 
            if let Some(order) = entry_count.get(&cost) {
                frontier.push(HeapEntry{ board: child, priority: cost, order: *order });
            }
        }
    }
}

fn bidirectional_solver(start_board: &Board, goal_board: &Board) {
    let start = Instant::now();
    let mut nodes_expanded = 0;
    let mut max_search_depth = 0;
    let mut solved       = false;
    let solution: Option<Board> = None;

    // Frontier is board states that we know exist but haven't explored yet
    let mut forward_frontier     = BinaryHeap::new();

    // Expored is board states we have compared to goal and expanded children
    let mut forward_explored     = HashSet::new();

    // Track collision count since multiple boards may have same priority and
    // you don't want to lose/replace boards.
    //
    // [P1] -> { B1, B2, B3 }
    // [P2] -> { B1 }
    // [P3] -> { B1, B2, B3, B4 }
    let mut entry_count  = HashMap::new();

    // Frontier is board states that we know exist but haven't explored yet
    let mut backward_frontier     = BinaryHeap::new();

    // Expored is board states we have compared to goal and expanded children
    let mut backward_explored     = HashSet::new();

    // Track collision count since multiple boards may have same priority and
    // you don't want to lose/replace boards.
    //
    // [P1] -> { B1, B2, B3 }
    // [P2] -> { B1 }
    // [P3] -> { B1, B2, B3, B4 }
    let mut bentry_count  = HashMap::new();

    // Make of copy of board since we are going to transfer ownership to priority queue
    let cloned_start = start_board.clone();
    let cloned_goal = goal_board.clone();

    // Give a fake priority to first board, we are going to pop it off the queue right awawy
    entry_count.insert(0, 1);
    forward_frontier.push(HeapEntry{ board: cloned_start, priority: 0, order: 1 });

    bentry_count.insert(0, 1);
    backward_frontier.push(HeapEntry{ board: cloned_goal, priority: 0, order: 1 });

    while !solved {
        if let Some(item) = forward_frontier.pop() {
            perform_move(item.board, &mut forward_frontier, &mut forward_explored, &mut entry_count);
        }

        if let Some(item) = backward_frontier.pop() {

        }
    }

    let duration = start.elapsed();

    if let Some(solution) = solution {
        let metrics = Metrics{
            path: solution.m_list,
            cost_of_path: solution.cost,
            nodes_expanded: nodes_expanded,
            max_search_depth: max_search_depth,
            search_depth: solution.cost,
            running_time: duration
        };

        println!("Solved Puzzle: {}", solved);
        metrics.display();
    }
}

fn solve(start_board : &Board, goal_board : &Board) {
    let start = Instant::now();
    let mut nodes_expanded = 0;
    let mut max_search_depth = 0;

    let mut frontier     = BinaryHeap::new();
    let mut explored     = HashSet::new();
    let mut entry_count  = HashMap::new();
    let mut solved       = false;

    let cloned_board = start_board.clone();
    let mut solution = None;
    entry_count.insert(0, 1);
    frontier.push(HeapEntry{ board: cloned_board, priority: 0, order: 1 });

    while !frontier.is_empty() {
        if let Some(item) = frontier.pop() {
            solved = test_goal(&item.board, goal_board);

            //println!("Looking at board on iteration: {}", nodes_expanded);
            //item.board.print();

            if solved { 
                solution = Some(item.board);
                break;
            }

            let key = calculate_hash(&item.board);
            explored.insert(key);

            let children = item.board.expand();
            for child in children {

                let val = calculate_hash(&child);
                if !explored.contains(&val) {
                    //println!("Looking at child");
                    //child.print();

                    let cost = calculate_move_cost(start_board, &child);
                    let key  = calculate_hash(&child);
                    max_search_depth = std::cmp::max(max_search_depth, child.cost);

                    *entry_count.entry(cost).or_insert(1) += 1;

                    if let Some(order) = entry_count.get(&cost) {
                        frontier.push(HeapEntry{ board: child, priority: cost, order: *order });
                        explored.insert(key);
                    }
                }
            }
            nodes_expanded += 1
        }
    }

    let duration = start.elapsed();

    if let Some(solution) = solution {
        let metrics = Metrics{
            path: solution.m_list,
            cost_of_path: solution.cost,
            nodes_expanded: nodes_expanded,
            max_search_depth: max_search_depth,
            search_depth: solution.cost,
            running_time: duration
        };

        println!("Solved Puzzle: {}", solved);
        metrics.display();
    }
}

fn parse_args(args: &[String]) -> [u8;16] {
    let mut state: [u8;16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    if args.len() != 16 {
        panic!("Not enough command line arguments supplied");
    }

    let mut index: usize = 0;

    for arg in args {
        state[index] = arg.parse::<u8>().unwrap();
        index += 1;
    }

    state
}

fn main() {
    // TODO - Implement valid board check function
    // Known Valid Boards...
    // 11 15 3 12 2 8 10 1 4 6 5 14 13 7 9 0

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let state = parse_args(&args);
    let end : [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];

    let start = Board {
        size: 4,
        state: state,
        m_list: Vec::new(),
        cost: 0
    };

    if !is_solvable(&start) {
        panic!("Board is not solvable, try another one!");
    }

    let goal = Board {
        size: 4,
        state: end,
        m_list: Vec::new(),
        cost: 0
    };

    start.print();
    solve(&start, &goal);
    //solve(&goal, &start);

    /*
    let moves = b.expand();

    for board in &moves {
        board.print();
        println!();
    }
    */
}
