import type {Move} from "chess.js";

// A standard positional input consists of 1 + (1 + 2 * 6) * 64 = 833 floats.
// The first float is 1.0 if the side to move is white, and 0.0 if black.
// The rest encodes the piece (or lack thereof) on each square.
// It consists of 13 * 64 floats:
// The first one is 1.0 if the field is empty, 0.0 else.
// The second one is 1.0 if the field is occupied by a white pawn, 0.0 else.
// The third one is 1.0 if the field is occupied by a white knight, 0.0 else.
// ...
// The twelfth one is 1.0 if the field is occupied by a black king, 0.0 else.
export type StandardPositionalInput = Float32Array & { length: 833 } & {
  readonly __tag: unique symbol;
};

// A from-to output contains 64 * 2 = 128 floats.
// The first 64 floats encode the from square, the second 64 the to square.
export type FromToOutput = Float32Array & { length: 128 } & {
  readonly __tag: unique symbol;
};

// A complete output contains 64 * 64 = 4096 floats.
// Every neuron encodes one move, they are lexicographically ordered.
// This means that the first 64 floats encode every move from a1, the next 64 from a2, and so on.
export type CompleteOutput = Float32Array & { length: 4096 } & {
  readonly __tag: unique symbol;
};

// An evaluation contains a single float.
export type Evaluation = Float32Array & { length: 1 } & {
  readonly __tag: unique symbol;
};

export type MoveWithAct = { from: string, to: string, act: number, inner ?: Move, index ?: number };
