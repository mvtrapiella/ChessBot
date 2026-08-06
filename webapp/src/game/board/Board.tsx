import { useState } from "react";
import Cell from "./Cell";

function Board() {
    const [board, setBoard] = useState(() => 
        new Array(8).fill(new Cell).map(() => new Array(8).fill(null))
    );

    const initialize_board = () => {

    }
    return (
        <></>
    )
}

export default Board