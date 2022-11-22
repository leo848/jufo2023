import {Chess, type Square} from "chess.js";

export let game = new Chess();

const events: ((game: Chess) => void)[] = [];

export function addEvent(event: (game: Chess) => void) {
  events.push(event);
}

export function move(move: string | {from: string; to: string; promotion?: string | undefined;}) {
  game.move(move);
  events.forEach(event => event(game));
}

export function isSquare(str: string): str is Square {
  if (str.length !== 2) return false;
  const col = str.charAt(0);
  const row = Number(str.charAt(1));
  return ['a','b','c','d','e','f','g','h'].includes(col) && row >= 1 && row <= 8
}
