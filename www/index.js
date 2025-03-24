// import * as wasm from "hello-wasm-pack";
import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

// wasm.greet("Filipiconst GRID_COLOR = "#CCCCCC";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// const pre = document.getElementById("game-of-life-canvas");

// const universe = Universe.new();
// const universe = Universe.blank();
const universe = Universe.random();
const width = universe.width();
const height = universe.height();

// const glider = Universe.glider();
// universe.place(glider, 1, 1);
// universe.place(glider, 10, 3);
// universe.place(glider, 13, 9);

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');


const getIndex = (row, column) => {
  return row * width + column
};


const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0); // x,y=0
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1); //x=0,y
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
};


const drawCells = () => {
  const cellPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellPtr, width * height / 8);
  // console.log(memory.buffer)
  // console.log(cells)

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col)

      // console.log(idx)
      // console.log(cells[idx])

      ctx.fillStyle = bitIsSet(idx, cells)
        ? DEAD_COLOR
        : ALIVE_COLOR;

      // console.log(ctx.fillStyle)



      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};



const renderLoop = () => {
  // pre.textContent = universe.render();
  universe.tick();

  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
};


drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
