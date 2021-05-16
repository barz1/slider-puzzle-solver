import TinyQueue from 'tinyqueue';
import hash from 'object-hash';

// Convert a move to a string
// 0 - UP
// 1 - DOWN
// 2 - LEFT
// 3 - RIGHT
function move_to_string(move) {
    switch(move) {
        case 0: return "UP";
        case 1: return "DOWN";
        case 2: return "LEFT";
        case 3: return "RIGHT";
        default: return "ERROR!";
    }
}

class Board {
    constructor(state, size, cost, parent, move) {
        // An array for the board state
        this.state = state;

        // The size of a row/column in board
        this.size  = size;

        // Cost (Number of moves/depth from starting board)
        this.cost  = cost;

        this.parent = parent;

        // Move from parent board to this board
        this.move  = move;
    }

    // Expand the current board into its child boards
    expand() {
        let moves = [];

        const up_move = this.move_up();
        if (up_move !== null) {
            moves.push(up_move);
        }

        const down_move = this.move_down();
        if (down_move !== null) {
            moves.push(down_move);
        }

        const left_move = this.move_left();
        if (left_move !== null) {
            moves.push(left_move);
        }

        const right_move = this.move_right();
        if (right_move !== null) {
            moves.push(right_move);
        }

        return moves;
    }

    move_up() {
        let zero_index = this.state.findIndex(value => value === 0);

        // Can't move up from top row
        if (zero_index < this.size) {
            return null;
        }

        // Swap empty tile with one above it
        let new_state = this.state.map(value => value);
        new_state[zero_index]             = new_state[zero_index - this.size];
        new_state[zero_index - this.size] = 0

        // Return a new board
        return new Board(new_state, this.size, this.cost+1, this, 0);
    }

    move_down() {
        let zero_index = this.state.findIndex(value => value === 0);

        // Can't move down from bottom row
        if (Math.floor(zero_index / this.size) === this.size-1) {
            return null;
        }

        // Swap empty tile with one below it
        let new_state = this.state.map(value => value);
        new_state[zero_index]             = new_state[zero_index + this.size];
        new_state[zero_index + this.size] = 0

        return new Board(new_state, this.size, this.cost+1, this, 1);
    }

    move_left() {
        let zero_index = this.state.findIndex(value => value === 0);

        // Can't move left from first column
        if (zero_index % this.size === 0) {
            return null;
        }

        // Swap empty tile with one to the left
        let new_state = this.state.map(value => value);
        new_state[zero_index] = new_state[zero_index - 1];
        new_state[zero_index - 1] = 0;

        return new Board(new_state, this.size, this.cost+1, this, 2);
    }

    move_right() {
        let zero_index = this.state.findIndex(value => value === 0);

        // Can't move right from last column
        if (zero_index % this.size === this.size -1) {
            return null;
        }

        // Swap empty tile with one to the right
        let new_state = this.state.map(value => value);
        new_state[zero_index] = new_state[zero_index + 1];
        new_state[zero_index + 1] = 0;

        return new Board(new_state, this.size, this.cost+1, this, 3);
    }
}


class Solver {

    // Solve the given board
    solve(board, goal) {

        // let queue consit of a 'Node'
        // {
        //   'priority':   ResultOfPriorityFunction,
        //   'occurrence': MonotonicallyIncreasingOccurrenceOfPriority,
        //   'board':      Board
        // }
        let queue = new TinyQueue([], function(lhs, rhs) {
            if (lhs.priority === rhs.priority) {
                return lhs.occurrence - rhs.occurrence
            }
            return lhs.priority - rhs.priority;
        });

        let solution    = null;
        let visited     = new Map();
        let occurrences = new Map();
        queue.push({ 'priority': 0, 'occurrence': 0, 'board': board});
        
        while (queue.data.length > 0) {
            let current = queue.pop();

            if (this.test_goal(current.board, goal)) {
                console.log("Solved!");
                solution = current.board;
                break;
            }

            const board_hash = hash(current.board.state);
            visited.set(board_hash, current.board);

            let children = current.board.expand();

            // Iterate over each 'child' board and see if we've seen it before or not
            children.forEach((child) => {

                // Use a hash of the board state to determine if this is a new board
                const child_hash = hash(child.state);

                if ( !visited.has(child_hash) ) {

                    // If new board, compute the board cost and occurrence of this board cost
                    const priority = this.calculate_cost(child, goal);
                    let   occurrence = 0;
                    if (occurrences.has(priority)) {
                        occurrence = occurrences.get(priority) + 1;
                        occurrences.set(priority, occurrence);
                    }

                    queue.push({ 'priority': priority, 'occurrence': occurrence, 'board': child});
                }
            });
        }

        return this.get_solution_path(solution);
    }

    get_solution_path(board) {
        let moves = [];

        while (board.parent !== null) {
            moves.push(board.move);
            board = board.parent;
        }
        return moves.reverse();
    }

    calculate_cost(board, goal) {
        return board.cost + this.calculate_manhattan_dist(board, goal);
    }

    calculate_manhattan_dist(board, goal) {
        let distance = 0;

        // Iterate over each tile in the board and compute its distance from its goal position
        board.state.forEach((value, index) => {
            const goal_index = goal.state.findIndex(goal_value => goal_value === value);
            distance += this.calculate_manhattan_dist_tile(index, goal_index, board.size);
        });

        return distance;
    }

    calculate_manhattan_dist_tile(current_index, goal_index, board_size) {
        const current_row = current_index / board_size;
        const current_col = current_index % board_size;
        const goal_row    = goal_index / board_size;
        const goal_col    = goal_index % board_size;

        return Math.abs(goal_row - current_row) + Math.abs(goal_col - current_col);
    }

    // Test if two boards are equal
    test_goal(board, goal) {
        for (let ii=0; ii<board.state.length; ii++) {
            if (board.state.[ii] !== goal.state[ii]) {
                return false;
            }
        }
        return true;
    }
}

export {Board, Solver, move_to_string};