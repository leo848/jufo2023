import {evaluationOutputToEvaluation, fenToStandardPositionalInput} from '@/neural-models/chess_conversions';
import type {Chess, Move} from 'chess.js';
import { loadModel } from '../neural-models/model';

const model = await loadModel("20mevaltrain-512neurons-4layers");

type Player = boolean;

export function evaluate(fen: string, player: Player) {
  const input = fenToStandardPositionalInput(fen);
  const evaluation = evaluationOutputToEvaluation(model.predict(input)) * 2 - 1;
  return player ? evaluation : -evaluation;
}

export function miniMax(game: Chess, depth: number): Move {
  const player = game.turn() === "w";
  let max = -Infinity;
  let bestMove: Move | null = null;

  for (const move of game.moves()) {
    const playedMove = game.move(move);
    const value = -miniMaxInternal(game, !player, depth - 1);
    game.undo();
    if (value > max) {
      max = value;
      bestMove = playedMove;
      console.error("New best move", move, value);
    }
  }

  if (bestMove === null) {
    throw new Error("No move found");
  }
  return bestMove;
}

export function miniMaxInternal(game: Chess, player: Player, depth: number): number {
  if (depth === 0) {
    return evaluate(game.fen(), player);
  }

  let max = -Infinity;
  for (const move of game.moves()) {
    game.move(move);
    const value = -miniMaxInternal(game, !player, depth - 1);
    game.undo();
    max = Math.max(max, value);
  }
  return max;
}
