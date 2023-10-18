import type {
  StandardPositionalInput,
  FromToOutput,
  MoveWithAct,
  CompleteOutput,
} from "./types";

import { Chess, SQUARES, type Piece, type Square } from "chess.js";
import { read, write } from "chessground/fen";
import type {Key, Pieces} from "chessground/types";

export function fenToStandardPositionalInput(
  fen: string
): StandardPositionalInput {
  let game = new Chess(fen);
  let input = [game.turn() === "w" ? 1.0 : 0.0];
  let board = game.board().reverse().flat();
  for (const piece of board) {
    let newInput = new Array(13).fill(0.0);
    if (piece === null) {
      newInput[0] = 1.0;
    } else {
      let offset = piece.color === "w" ? 0 : 6;
      switch (piece.type) {
        case "p":
          newInput[1 + offset] = 1.0;
          break;
        case "n":
          newInput[2 + offset] = 1.0;
          break;
        case "b":
          newInput[3 + offset] = 1.0;
          break;
        case "r":
          newInput[4 + offset] = 1.0;
          break;
        case "q":
          newInput[5 + offset] = 1.0;
          break;
        case "k":
          newInput[6 + offset] = 1.0;
          break;
        default:
          console.error("Error: invalid piece type");
      }
    }
    input = input.concat(newInput);
  }
  if (input.length !== 1 + (1 + 6 * 2) * 64) {
    console.error(
      "Invalid length",
      input.length,
      "expected",
      1 + (1 + 6 * 2) * 64
    );
  }
  return new Float32Array(input) as StandardPositionalInput;
}

function quantifyBoolean(input: number): boolean {
  return input > 0.5;
}

export function standardPositionalInputToFen(input: StandardPositionalInput): string {
  let field: Pieces = new Map();
  let index = 0;
  const turn = quantifyBoolean(input[index++]);
  
  for (const rank of ["1", "2", "3", "4", "5", "6", "7", "8"] as const) {
    for (const file of ["a", "b", "c", "d", "e", "f", "g", "h"] as const) {
      const square = file + rank;
      const relevant = input.slice(index, index + 13);
      let maxIndex = 0, max = -Infinity;
      relevant.forEach((v, i) => { if (v > max) { max = v; maxIndex = i }});
      if (maxIndex != 0) {
        let pieceType = (maxIndex - 1) % 6, pieceColor = Math.floor((maxIndex - 1) / 6);
        field.set(square as Square, {
          role: (['pawn', 'knight', 'bishop', 'rook', 'queen', 'king'] as const)[pieceType],
          color: (['white', 'black'] as const)[pieceColor],
        })
      }
      index += 13;
    }
  }

  let cgFen = write(field);
  // if (!turn) cgFen = cgFen.replace("w", "b");
  return cgFen;
}

export function fenDifference(left: string, right: string): { ratio: number, deltas: Square[] } {
  let piecesL = read(left), piecesR = read(right);
  let errors: Square[] = [];
  let count = 0, totalError = 0;
  for (const square of SQUARES) {
    if (piecesL.get(square) || piecesR.get(square)) {
      count += 1;
    } else continue;
    const g = (ps: Pieces) => ps.get(square as Key) || {} as Record<any, any>;
    let pieceL = g(piecesL), pieceR = g(piecesR);
    if (pieceL.color + pieceL.role != pieceR.color + pieceR.role) {
      errors.push(square);
      totalError += 1;
    }
  }
  if (count == 0) count = 1;
  return { ratio: totalError / count, deltas: errors };
}

export function fromToOutputToMoves(
  output: FromToOutput,
  options: { amount: number } = { amount: 10 }
): MoveWithAct[] {
  const { amount } = options;

  let [fromMoves, toMoves] = [
    [...output.slice(0, 64)],
    [...output.slice(64, 128)],
  ].map((arr) => {
    return arr
      .map((act, i) => [act, i])
      .map(([act, i]) => [act, i % 8, Math.floor(i / 8)]);
  });

  let moves = fromMoves
    .map(([act1, x1, y1]) => {
      return toMoves.map(([act2, x2, y2]) => {
        return [act1 * act2, [x1, y1], [x2, y2]] as const;
      });
    })
    .flat()
    .sort((a, b) => b[0] - a[0])
    .slice(0, amount)
    .map(([act, from, to]) => {
      let intToChar = (i: number) => String.fromCharCode(97 + i);
      return {
        act,
        from: intToChar(from[0]) + (from[1] + 1),
        to: intToChar(to[0]) + (to[1] + 1),
      };
    });

  return moves;
}

export function completeOutputToMoves(
  output: CompleteOutput,
): MoveWithAct[] {
  let moves = [...output]
    .map((act, i) => [act, [i / 64, i % 64]] as const)
    .map(([act, [from, to]]) => {
      let intToChar = (i: number) => String.fromCharCode(97 + i);
      return {
        act,
        from: intToChar(from % 8) + (Math.floor(from / 8) + 1),
        to: intToChar(to % 8) + (Math.floor(to / 8) + 1),
      };
    });

  return moves.sort((a, b) => b.act - a.act);
}

export function evaluationOutputToEvaluation(
  output: Float32Array
): number {
  return output[0];
}
