import type {
  StandardPositionalInput,
  FromToOutput,
  MoveWithAct,
  CompleteOutput,
} from "./types";

import { Chess } from "chess.js";

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
