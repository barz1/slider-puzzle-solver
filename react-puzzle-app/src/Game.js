import Tile from './Tile.js';
import './board.css'
import { useEffect, useState} from 'react';
import { Button } from '@material-ui/core';
import { Solver, Board, move_to_string} from './Solver.js';

// TODO - Randomly generate board and verify it is solvable
const generateBoard = (size) => {
    let values = [ ...Array(size*size).keys() ];
    let board  = [];
    let solvable = false;
    console.log(values);

    while ( !solvable ) {
        while (values.length !== 0) {
            let random_index = Math.floor(Math.random() * Math.floor(values.length));
            board.push(values[random_index]);

            values.splice(random_index, 1);
        }

        console.log(board);
        solvable =  isSolvable(board, size);
        console.log("Solvable: " + solvable);
        if ( !solvable ) {
            let values = board.map(entry => entry);
            //board = [1, 2, 3, 0, 5, 6, 4, 7, 8];
            //solvable = true;
        }
    }

    return board;
}

const isSolvable = (board, size) => {
    let inversions = 0;
    let zero_index = 0;

    for (let ii=0; ii<board.length; ii++) {
        const currentValue = board[ii];

        // Check if current tile is blank square, record index and skip inversions count
        if (currentValue === 0) {
            zero_index = ii;
            continue;
        }

        for (let jj=ii+1; jj<board.length; jj++) {
            if (board[jj] !== 0 && currentValue > board[jj]) {
                inversions += 1;
            }
        }
    }

    console.log("Inversions: " + inversions);
    // If odd sized board (odd number of rows/columns) then inversion must be even to be solvable
    if (size % 2 !== 0) {
        return (inversions % 2 === 0);
    }
    // For even sized boards, solvable if inversions plus row of blank square is odd
    else {
        return ((inversions + (zero_index / size)) % 2 !== 0);
    }
}

export function Game(props) {
    const size = 3;

    // Track original board so 'Rest' functionality works
    const [originalBoard, setOriginalBoard] = useState(() => generateBoard(size));

    // Track current board
    const [numbers, setNumbers]   = useState(originalBoard.map(entry => entry));

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

    // Callback when user requests a new game
    const newGame = () => {
        const newGame = generateBoard(size);
        setOriginalBoard(newGame);
        setNumbers(newGame.map(entry => entry));
    }

    // Callback when user requests to reset current game
    const reset = () => {
        setSolution(null);
        setSolMove(0);
        setNumbers(originalBoard.map(entry => entry));
    }

    // Callback for when the user requests to have the current board solved
    const solveBoard = () => {
        const start = new Board(numbers, size, 0, null);
        const goal  = new Board([1, 2, 3, 4, 5, 6, 7, 8, 0], size, 0, null, null);

        const solver = new Solver();
        setSolution(solver.solve(start, goal));
    }

    useEffect(() => {
        // Only call this when we have a solution (Ex. user clicked 'Solve')
        // and we haven't iterated through all moves in solution list yet
        if (solution !== null && solMove !== solution.length) {

            // Introduce a sleep between moves so user can see them happen
            const timer = setTimeout(() => {
                const move       = solution[solMove];
                const zero_index = numbers.findIndex(entry => entry ===0);
                let   index      = 0;

                // This logic is a little strange. From a players perspective, they are
                // trying to move a squre into the empty space.
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
            <div className={"board"}>
                {numbers.map((value, index) => (
                    <Tile key={value} value={value} index={index} size={numbers.length} handleClick={moveTile} /> 
                ))}
            </div>
        </div>
        <div className={"controls"}>
            <Button variant='contained' onClick={newGame}>New Game</Button>
            <Button variant='contained' onClick={reset}>Reset</Button>
            <Button variant='contained' onClick={solveBoard} disabled={solution !== null}>Solve</Button>
        </div>
    </div>
}