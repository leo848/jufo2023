<template>
  <v-row>
    <v-col cols="12">
      <p class="text-h5">Zugvorschläge des Modells</p>
      <p class="text-body-2">
        Sicherheit: {{ (gini * 100).toFixed(2) }}%
      </p>
    </v-col>
    <transition-group name="moves">
    <v-col
      v-for="move in computedMoves"
      :key="move.from + move.to + move.inner?.san"
      cols="6" sm="12"
    >
    <v-card
      :style="'background: ' + (move.inner ? actToColor(move.act) : gray)"
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
          <div v-if="showActivation" class="text-gray mr-1">
            {{ move.act.toFixed(4).slice(1) }}<br/>
            <div v-if="showIndex" class="mt-n2">#{{ move.index }}</div>
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
import { loadSetting } from "@/settings/settings";

export default {
  name: "MoveDisplay",
  data: () => ({
    gray: "grey lighten-3",
    showIndex: loadSetting("onlyShowLegalMoves"),
    showActivation: loadSetting("showActivation"),
  }),
  computed: {
    computedMoves() {
      let maxMoves = loadSetting("maxMoves");
      return this.moves.slice(0, maxMoves);
    }
  },
  methods: {
    interpolate: (t: number) => (x: number) =>
      1 + 1 / t - 1 / (t * x + 1) - 1 / (t * t * x + t),
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
      // return `rgb(${red}, ${green}, 0)`;

      // Create a linear gradient with the calculated color on the left and a darker version on the right.
      // The first color stop is at 75% and the second at 90%.
      return `linear-gradient(90deg, rgb(${red}, ${green}, 0) 70%, rgb(${red * 0.7}, ${green * 0.7}, 0) 90%)`;
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
    gini: {
      type: Number,
      required: true,
    },
  },
};
</script>
