import {Chess} from "chess.js";

export let game = new Chess();

const events: ((game: Chess) => void)[] = [];

export function addEvent(event: (game: Chess) => void) {
  events.push(event);
}

export function move(move: string | {from: string; to: string; promotion?: string | undefined;}) {
  game.move(move);
  events.forEach(event => event(game));
}
