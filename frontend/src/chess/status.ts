import type { Chess } from "chess.js";

export type Status = {
  result: ResultType;
  reason: Reason;
};

type ResultType = "1-0" | "0-1" | "1/2-1/2" | "*";

type Reason =
  | "checkmate"
  | "resign"
  | "stalemate"
  | "timeout"
  | "insufficient"
  | "agreement"
  | "repetition"
  | "50move"
  | "variantend"
  | "other";

export function getStatus(game: Chess): Status {
  if (game.isCheckmate()) {
    return {
      result: game.turn() === "w" ? "0-1" : "1-0",
      reason: "checkmate",
    };
  } else if (game.isDraw()) {
    return {
      result: "1/2-1/2",
      reason: game.isStalemate()
        ? "stalemate"
        : game.isThreefoldRepetition()
        ? "repetition"
        : game.isInsufficientMaterial()
        ? "insufficient"
        : "other",
    };
  } else {
    return {
      result: "*",
      reason: "other",
    };
  }
}
