import {Chess, type Square} from "chess.js";

export let game = new Chess();

const events: Record<number, ((game: Chess) => void)> = {};

let number = 69;

export function addEvent(event: (game: Chess) => void): number {
  events[number] = event;
  for (const event of Object.values(events)) {
    event(game);
  }
  return number++;
}

export function removeEvent(id: number) {
  delete events[id];
}

export function runEvent(id: number) {
  events[id](game);
}

export function move(move: string | {from: string; to: string; promotion?: string | undefined;}) {
  game.move(move);
  for (const event of Object.values(events)) {
    event(game);
  }
}

export function setFen(fen: string) {
  game.load(fen);
  for (const event of Object.values(events)) {
    event(game);
  }
}

export function resetGame() {
  game.reset();
  for (const event of Object.values(events)) {
    event(game);
  }
}

export function isSquare(str: string): str is Square {
  if (str.length !== 2) return false;
  const col = str.charAt(0);
  const row = Number(str.charAt(1));
  return ['a','b','c','d','e','f','g','h'].includes(col) && row >= 1 && row <= 8
}
