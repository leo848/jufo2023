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

export function translateReason(reason: Reason): string {
  if (reason === "checkmate") return "Schachmatt";
  if (reason === "resign") return "Aufgabe";
  if (reason === "stalemate") return "Patt";
  if (reason === "timeout") return "Zeit abgelaufen";
  if (reason === "insufficient") return "Zu wenig Material";
  if (reason === "repetition") return "Dreifache Stellungswiederholung";
  if (reason === "agreement") return "Absprache";
  if (reason === "50move") return "50 Züge nichts passiert";
  if (reason === "variantend") return "Variantenende";
  if (reason === "other") return "Unbekannter Grund";
  return assertNever(reason);
}

function assertNever<T>(assertion: never): T {
  throw new Error("this should *never* happen");
}

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
        : "agreement",
    };
  } else {
    return {
      result: "*",
      reason: "other",
    };
  }
}
