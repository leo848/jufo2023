<template>
  <v-row>
    <v-col
      v-for="move in moves"
      :key="move.from + move.to + move.inner?.san"
      cols="6" sm="12"
    >
      <v-fade-transition>
        <v-sheet
          :color="move.inner ? actToColor(move.act) : gray"
          :key="move.from + move.to + move.inner?.san"
          max-width="300"
          max-height="50"
          elevation="3"
          rounded="lg"
          >
          <div v-if="move.inner" v-html="prettyMove(move.inner)" />
          <div v-else>{{ move.from }} - {{ move.to }}</div>
          </v-sheet>
      </v-fade-transition>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import type { Move } from "chess.js";
import { loadPiece } from "@/chess/loadPieces";

export default {
  name: "MoveDisplay",
  data: () => ({
    gray: "grey lighten-3",
    interpolate: (t: number) => (x: number) =>
      1 + 1 / t - 1 / (t * x + 1) - 1 / (t * t * x + t),
  }),
  created() {
    console.log(this.moves);
  },
  methods: {
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
      let red = Math.min(1.0, 2.0 - 2.0 * newAct) * 255;
      let green = Math.min(1.0, 2.0 * newAct) * 255;
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
  width: 1.5em;
  transform: scale(1.5);
  margin-left: 0.5em;
  margin-right: 0.5em;
}
</style>
