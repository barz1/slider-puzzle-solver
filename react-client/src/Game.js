import Tile from './Tile.js';
import './board.css'
import { useEffect, useState} from 'react';
import { Button, MenuItem, Select } from '@material-ui/core';
import { Solver, GameBoard } from './Solver.js';


export function Game(props) {
    // Default to a 3x3 board
    const [size, setSize] = useState(3);

    // Handler for the wasm functions
    const [backend, setBackend] = useState(null);

    // Track original board so 'Reset' functionality works
    const [originalBoard, setOriginalBoard] = useState([1,2,3,4,5,6,7,8,9,0]);

    // Track current board
    const [numbers, setNumbers]   = useState([]);

    // Track the solution to the current board
    const [solution, setSolution] = useState(null);

    // For solving logic, track which move in the solution list we are at
    const [solMove, setSolMove]   = useState(0);

    // Determine if the current move is valid. A valid move is from any
    // tile adjacent to the empty square
    const isMoveValid = (index, zero_index) => {

                // Move up
        return ((zero_index > size-1 && zero_index - size === index) ||
                // Move down
                (Math.floor(zero_index / size) < size - 1 && zero_index + size === index) ||
                // Move left
                (zero_index % size > 0 && zero_index - 1 === index) ||
                // Move right
                (zero_index % size < size - 1 && zero_index + 1 === index))
    }

    // Callback for when a user clicks a tile
    const moveTile = (index) => {
        const value      = numbers[index];
        const zero_index = numbers.findIndex(entry => entry === 0);

        if (isMoveValid(index, zero_index)) {
            let new_numbers = numbers.map(value => value);

            new_numbers[index] = 0;
            new_numbers[zero_index] = value;
            console.log(new_numbers);

            setNumbers(new_numbers);
        }
    }

    const updateSize = (event) => {
        // Ignore if no change in size
        if (size === event.target.value) { return; }

        setSize(event.target.value);
        newGame(event.target.value);
    }

    // Callback when user requests a new game
    const newGame = (newSize) => {
        const board = backend.Board.generate_board(newSize);
        const state = Array.from(board.get_state());
        setSize(newSize);
        setOriginalBoard(board);
        setSolution(null);
        setSolMove(0);
        setNumbers(state.map(entry => entry));
    }

    // Callback when user requests to reset current game
    const reset = () => {
        setSolution(null);
        setSolMove(0);
        setNumbers(originalBoard.get_state().map(entry => entry));
    }

    // Callback for when the user requests to have the current board solved
    const solveBoard = () => {
        const start = new GameBoard(numbers, size, 0, null);

        let goalState = [1, 2, 3, 4, 5, 6, 7, 8, 0];
        if (size === 4) {
            goalState = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        }

        const goal  = new GameBoard(goalState, size, 0, null, null);
        console.log(originalBoard.render());

        backend.bidirectional_solver(originalBoard);

        const solver = new Solver();
        setSolution(solver.solve(start, goal));
    }

    // Make this call on initial load to load the wasm module
    useEffect(() => {

        async function load_wasm() {
            let module = await import('wasm-test');
            setBackend(module);

            const board = module.Board.generate_board(size);
            const state = Array.from(board.get_state());
            setOriginalBoard(board);
            setNumbers(state.map(entry => entry));
        };
        load_wasm();
    }, []);

    useEffect(() => {
        // Only call this when we have a solution (Ex. user clicked 'Solve')
        // and we haven't iterated through all moves in solution list yet
        if (solution !== null && solMove !== solution.length) {

            // Introduce a sleep between moves so user can see them happen
            const timer = setTimeout(() => {
                const move       = solution[solMove];
                const zero_index = numbers.findIndex(entry => entry === 0);
                let   index      = 0;

                // This logic is a little strange. From a players perspective, they are
                // trying to move a square into the empty space.
                // From the solver's perspective it is trying to move the empty square around.
                // So here we just swap the index of the empty space with where we are trying to move it.
                if (move === 0) {
                    index = zero_index - size;
                }
                else if (move === 1) {
                    index = zero_index + size
                }
                else if (move === 2) {
                    index = zero_index - 1;
                }
                else {
                    index = zero_index + 1;
                }

                let new_numbers = numbers.map(value => value);

                new_numbers[zero_index] = new_numbers[index];
                new_numbers[index] = 0;
                console.log(new_numbers);

                setNumbers(new_numbers);
                setSolMove(solMove + 1);
            }, 500);

            return () => clearTimeout(timer);
        }
    }, [numbers, solution, solMove]);


    return <div>
        <h1>{props.name}</h1>
        <div className={"board-container"}>
            <div className={`board-${size}`}>
                {numbers.map((value, index) => (
                    <Tile key={value} value={value} index={index} size={numbers.length} handleClick={moveTile} /> 
                ))}
            </div>
        </div>
        <div className={"controls"}>
            <Button variant='contained' onClick={() => newGame(size)}>New Game</Button>
            <Button variant='contained' onClick={reset}>Reset</Button>
            <Button variant='contained' onClick={solveBoard} disabled={solution !== null}>Solve</Button>
            <label htmlFor='board-size-select'>Select Board Size</label>
            <Select id='board-size-select' value={size} onChange={updateSize}>
                <MenuItem value={3}>3x3</MenuItem>
                <MenuItem value={4}>4x4</MenuItem>
            </Select>
        </div>
    </div>
}
