<template>
  <v-row>
    <transition-group name="moves">
    <v-col
      v-for="move in moves"
      :key="move.from + move.to + move.inner?.san"
      cols="6" sm="12"
    >
    <v-card
      :color="move.inner ? actToColor(move.act) : gray"
      :key="move.from + move.to + move.inner?.san"
      @click="move.inner && makeMove(move.inner)"
      max-width="300"
      max-height="50"
      elevation="3"
      rounded="lg"
      >
      <div class="d-flex flex-row justify-space-between">
        <div v-if="move.inner" v-html="prettyMove(move.inner)" />
          <div v-else class="text-h5 pl-2 py-1">{{ move.from }} - {{ move.to }}</div>
          <div class="text-gray mr-1">
            {{ move.act.toFixed(4).slice(1) }}
          </div>
        </div>
    </v-card>
    </v-col></transition-group>
  </v-row>
</template>

<script lang="ts">
import type { Move } from "chess.js";
import { loadPiece } from "@/chess/loadPieces";
import { move } from "@/chess/game";

export default {
  name: "MoveDisplay",
  data: () => ({
    gray: "grey lighten-3",
    interpolate: (t: number) => (x: number) =>
      1 + 1 / t - 1 / (t * x + 1) - 1 / (t * t * x + t),
  }),
  methods: {
    makeMove(m: Move) {
      move(m);
    },
    prettyMove(move: Move): string {
      let san = move.san;
      if (move.piece !== "p" && !san.startsWith("O")) san = san.slice(1);
      const imgSrc = loadPiece(move.color + move.piece.toUpperCase());
      return `
      <div class="d-flex py-1">
        <img class="piece-letter" src="${imgSrc}"/> <span class="text-h5">${san}</span>
      </div>
        `;
    },
    actToColor(act: number): string {
      // Return a color based on the activation value.
      // The color is a gradient from red to green.
      // It is red when the activation is 0 and green when the activation is 1.
      // The gradient is logarithmic.
      let newAct = this.interpolate(25)(act);
      let red = Math.min(1.0, 2.0 - 2.0 * newAct) * 200;
      let green = Math.min(1.0, 2.0 * newAct) * 200;
      return `rgb(${red}, ${green}, 0)`;
    },
  },
  props: {
    moves: {
      type: Array,
      required: true,
      validator: (moves: any) => {
        return moves.every((move: any) => {
          return move.from && move.to && move.act;
        });
      },
    },
  },
};
</script>

<style>
.piece-letter {
  height: 1.5em;
  width: 1.5em;
  margin-top: 0.25em;
  transform: scale(1.5);
  margin-left: 0.5em;
  margin-right: 0.5em;
}

.moves-move,
.moves-enter-active,
.moves-leave-active {
  transition: all 0.25s ease;
}
.moves-enter-from,
.moves-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.moves-leave-active {
  position: absolute;
  z-index: -1;
}


</style>
