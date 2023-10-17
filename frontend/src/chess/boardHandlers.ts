import type {Square} from "chess.js";
import { game } from "./game";

// Declare the Chessboard. This is any because we don't have the type definitions for chessboard.js
export declare const Chessboard: {
  constructor: (id: string, config: any) => any
};

export function getMove(input: string | {from: string; to: string; promotion?: string | undefined;}) {
  if (typeof input !== "string") {
    // If the input is a pawn promotion, set the promotion to 'q'.
    const piece = game.get(input.from as Square);
    if (piece && piece.type === "p" && (input.to[1] === "1" || input.to[1] === "8")) {
      input.promotion = "q";
    }
  }
  let move;
  try {
    move = game.move(input);
    game.undo();
  } catch (e) {
    return null;
  }
  return move;
}
