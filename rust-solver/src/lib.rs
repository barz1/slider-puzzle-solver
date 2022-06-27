#![feature(map_first_last)]
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
//use std::time::{Duration, Instant};

mod heuristics;
use self::heuristics::Heuristic;

mod board;
pub use self::board::{Board, Move};

/*
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
*/

fn add_entry(board: Board, board_collection: &mut BTreeMap<i32, Vec<Board>>) {
    match board_collection.entry(board.score) {
        Entry::Occupied(mut entries) => {
            let mut found = false;
            for entry in entries.get() {

                // Board already exists, don't re-add
                if *entry == board {
                    found = true;
                    break;
                }
            }

            if !found {
                entries.get_mut().push(board);
            }
        },

        // If score (priority) is vacant, just add child board
        Entry::Vacant(entry) => {
            entry.insert(vec!(board));
        },
    };

}

fn check_for_solution(new_moves: &Vec<Board>, frontier: &BTreeMap<i32, Vec<Board>>, explored: &BTreeMap<i32, Vec<Board>>) -> Option<(Vec<Move>, Vec<Move>)> {
    for board in new_moves {
        let mut native_state : [u8; 16] = [0; 16];
        for ii in 0..16 as i32 {
            native_state[ii as usize] = 0;

        }

        // Look through frontier boards
        if let Some(frontier_boards) = frontier.get(&board.score) {
            for frontier_board in frontier_boards {
                if *board == *frontier_board {
                    return Some((board.m_list.clone(), frontier_board.m_list.clone()));
                }
            }
        }

        // Look through explored boards
        if let Some(explored_boards) = explored.get(&board.score) {
            for explored_board in explored_boards {
                if *board == *explored_board {
                    return Some((board.m_list.clone(), explored_board.m_list.clone()));
                }
            }
        }
    }
    None
}

fn perform_move(frontier: &mut BTreeMap<i32, Vec<Board>>, explored: &mut BTreeMap<i32, Vec<Board>>, heuristics: &Heuristic) -> Vec<Board> {
    let mut children: Vec<Board> = Vec::new();

    // Get the next best priority board to expand
    if let Some(mut item) = frontier.first_entry() {

        let board = item.get_mut().remove(0);
        //board.print();

        children  = board.expand(heuristics);

        // Add board to the explored list
        add_entry(board, explored);

        // Only keep the board which haven't been explored yet
        children.retain(|entry| {
            if let Entry::Occupied(entries) = explored.entry(entry.score) {
                for explored_board in entries.get() {
                    if *explored_board == *entry {
                        return false;
                    }
                }
            }
            true
        });

        // Remove this priority from the queue if no more boards
        if item.get().is_empty() {
            item.remove_entry();
        }
    }

    // Now iterate over remaining child boards and add to frontier if they aren't there already
    for child in &mut children {
        add_entry(child.clone(), frontier);
    }

    children
}

pub fn bidirectional_solver(start_board: &Board, goal_board: &Board) -> Vec<Move> {
    //let start = Instant::now();
    let mut nodes_expanded = 0;
    let mut solution : Vec<Move> = Vec::new();

    //let mut max_search_depth = 0;
    //let solution: Option<Board> = None;

    // Frontier is board states that we know exist but haven't explored yet
    let mut forward_frontier:BTreeMap<i32, Vec<Board>> = BTreeMap::new();

    // Expored is board states we have compared to goal and expanded children
    let mut forward_explored: BTreeMap<i32, Vec<Board>> = BTreeMap::new();

    // Frontier is board states that we know exist but haven't explored yet
    let mut backward_frontier:BTreeMap<i32, Vec<Board>> = BTreeMap::new();

    // Expored is board states we have compared to goal and expanded children
    let mut backward_explored : BTreeMap<i32, Vec<Board>> = BTreeMap::new();

    // Make of copy of board since we are going to transfer ownership to priority queue
    let cloned_start = start_board.clone();
    let cloned_goal  = goal_board.clone();

    // Create the heuristics object to use
    let heuristics = Heuristic::new(start_board.clone()) ;

    // Give a fake priority to first board, we are going to pop it off the queue right away
    forward_frontier.insert(0, vec!(cloned_start));
    backward_frontier.insert(0, vec!(cloned_goal));

    let mut forward_found = false;
    let mut backward_found = false;

    while !forward_found && !backward_found {
        let forward_moves = perform_move(&mut forward_frontier, &mut forward_explored, &heuristics);
        let backward_moves = perform_move(&mut backward_frontier, &mut backward_explored, &heuristics);

        nodes_expanded = nodes_expanded + forward_moves.len() + backward_moves.len();

        let forward_solution = check_for_solution(&forward_moves, &backward_frontier, &backward_explored);
        if let Some(moves) = forward_solution {
            forward_found = true;

            //println!("{:?}  {:?}", moves.0, moves.1);

            solution = moves.0;

            let mut rest = moves.1.clone();
            rest.reverse();
            for backward_move in rest {
                match backward_move {
                    Move::Down => solution.push(Move::Up),
                    Move::Up => solution.push(Move::Down),
                    Move::Right => solution.push(Move::Left),
                    Move::Left => solution.push(Move::Right),
                    Move::None => ()
                }
            }
        }

        let backward_solution = check_for_solution(&backward_moves, &forward_frontier, &forward_explored);
        if let Some(moves) = backward_solution {
            backward_found = true;

            solution = moves.1;

            let mut rest = moves.0.clone();
            rest.reverse();
            for backward_move in rest {
                match backward_move {
                    Move::Down => solution.push(Move::Up),
                    Move::Up => solution.push(Move::Down),
                    Move::Right => solution.push(Move::Left),
                    Move::Left => solution.push(Move::Right),
                    Move::None => ()
                }
            }
        }
    }


    //let duration = start.elapsed();
    //println!("Solution found, duration: {:?}, nodes: {}", duration, nodes_expanded);

    /* 
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
    */

    solution

}

