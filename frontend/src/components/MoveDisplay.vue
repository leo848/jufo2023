<template>
  <v-row>
    <v-col cols="12">
      <p class="text-h5">
        Zugvorschläge des Modells <InfoBox name="move-suggestions" />
      </p>
      <p class="text-body-2">
        <v-btn
          :icon="movesShown ? 'mdi-eye-off-outline' : 'mdi-eye-outline'"
          @click="showAll"
          @blur="movesShown && showAll"
          :color="movesShown ? 'white' : undefined"
          variant="plain"
          class="mb-0"
        />
        Legale Ausgaben:
        <PercentageDial :value="legals" /> 
        <InfoBox class="ml-2" name="legal-outputs" :templates="{ percentage: (legals * 100).toFixed(2) }" />
      </p>
    </v-col>
    <transition-group name="moves">
      <v-col
        v-for="move in computedMoves"
        :key="move.from + move.to + move.inner?.san"
        cols="6"
        sm="12"
      >
        <v-card
          :style="'background: ' + (move.inner ? actToColor(move.act) : gray)"
          :key="move.from + move.to + move.inner?.san"
          @click="move.inner && makeMove(move.inner)"
          @mouseover="hover(move)"
          @mouseleave="hover()"
          max-width="300"
          max-height="50"
          elevation="3"
          rounded="lg"
        >
          <div class="d-flex flex-row justify-space-between">
            <div v-if="move.inner && prettyMove(move.inner)" v-html="prettyMove(move.inner)"></div>
            <div v-else class="text-h5 pl-2 py-1">
              {{ move.from }} - {{ move.to }}
            </div>
            <v-spacer />
            <div
              v-if="showActivation"
              class="text-gray mr-1"
              @click="hover(move)"
            >
              {{ move.act.toFixed(4).slice(1) }}<br />
              <div v-if="showIndex" class="mt-n2">#{{ move.index }}</div>
            </div>
            <InfoBox name="move-display" :templates="{ move }" />
          </div>
        </v-card>
      </v-col></transition-group
    >
  </v-row>
</template>

<script lang="ts">
import type { Move } from "chess.js";
import { loadPiece } from "@/chess/loadPieces";
import { move } from "@/chess/game";
import { loadSetting } from "@/settings/settings";
import type { MoveWithAct } from "@/neural-models/types";

import InfoBox from "@/components/InfoBox.vue";
import PercentageDial from "@/components/PercentageDial.vue";

export default {
  name: "MoveDisplay",
  components: {
    InfoBox,
    PercentageDial
  },
  data: () => ({
    gray: "grey lighten-3",
    showIndex: loadSetting("onlyShowLegalMoves"),
    showActivation: loadSetting("showActivation"),
    movesShown: false,
  }),
  computed: {
    computedMoves() {
      let maxMoves = loadSetting("maxMoves");
      let showOnlyLegalMoves = loadSetting("onlyShowLegalMoves");
      let moves = this.moves as (MoveWithAct & { inner?: Move })[];
      if (showOnlyLegalMoves) {
        moves = moves.filter((move) => move.inner);
      }
      return moves.slice(0, maxMoves);
    },
  },
  methods: {
    interpolate: (t: number) => (x: number) =>
      1 + 1 / t - 1 / (t * x + 1) - 1 / (t * t * x + t),
    makeMove(m: Move) {
      move(m);
    },
    prettyMove(move: Move): string {
      let san = move.san;
      if (!san) return "";
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
      let newAct = this.interpolate(20)(act);
      let red = Math.min(1.0, 2.0 - 2.0 * newAct) * 200;
      let green = Math.min(1.0, 2.0 * newAct) * 200;

      return `linear-gradient(90deg, rgb(${red}, ${green}, 0) 70%, rgb(${
        red * 0.7
      }, ${green * 0.7}, 0) 90%)`;
    },
    hover(move?: Move) {
      this.$emit("suggest", move);
      if (!move) {
        if (this.movesShown) {
          this.movesShown = false;
          this.showAll();
        }
      }
    },
    showAll() {
      if (this.movesShown) {
        this.movesShown = false;
        this.$emit("suggest", undefined);
        return;
      }
      this.movesShown = true;
      this.$emit("showAll");
    },
  },
  emits: ["suggest", "showAll"],
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
    legals: {
      type: Number,
      required: true,
    },
  },
};
</script>
