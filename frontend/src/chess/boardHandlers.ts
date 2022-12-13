import { game } from "./game";

// Declare the Chessboard. This is any because we don't have the type definitions for chessboard.js
export declare const Chessboard: {
  constructor: (id: string, config: any) => any
};

export function getMove(input: string | {from: string; to: string; promotion?: string | undefined;}) {
  if (typeof input !== "string") {
    if (input.promotion === undefined) {
      input.promotion = "q";
    }
  }
  let move = game.move(input);
  if (move != null) game.undo();
  return move;
}
