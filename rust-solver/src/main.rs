use std::env;
use std::time::Instant;

use slider_solver_lib::{Board, bidirectional_solver};

/*
fn get_entry(board: &Board, collection: &mut BTreeMap<i32, Vec<Board>>) -> Option<Board> {

    let parent = board.clone();
    match (parent.m)
    let cost = calculate_f_val(board);

    if let Entry::Occupied(entries) = collection.entry(cost) {
        for entry in entries.get() {
            if *entry == *board {
                return Some(entry.clone());
            }
        }
    }
    None
}
*/

/* 
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

            if solved { 
                solution = Some(item.board);
                break;
            }

            let key = calculate_hash(&item.board);
            explored.insert(key);

            let children = item.board.expand();
            for mut child in children {

                let val = calculate_hash(&child);
                if !explored.contains(&val) {
                    //println!("Looking at child");
                    //child.print();

                    let cost = calculate_move_cost(start_board, &mut child);
                    let key  = calculate_hash(&child);
                    max_search_depth = std::cmp::max(max_search_depth, child.cost);

                    *entry_count.entry(cost.2).or_insert(1) += 1;

                    if let Some(order) = entry_count.get(&cost.2) {
                        frontier.push(HeapEntry{ board: child, priority: cost.2, order: *order });
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
*/

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
        cost: 0,
        score: 0,
    };


    let goal = Board {
        size: 4,
        state: end,
        m_list: Vec::new(),
        cost: 0,
        score: 0
    };

    //start.print();
    //println!();
    //goal.print();

    if !start.is_solvable() {
        panic!("Board is not solvable, try another one!");
    }

    let start_time = Instant::now();

    let solution = bidirectional_solver(&start, &goal);

    let duration = start_time.elapsed();
    start.print_flat();
    println!(", {:?}, {:?}", duration, solution);


}
