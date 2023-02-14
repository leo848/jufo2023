<template>
<v-card>
  <v-card-title>Neuronale Fortsetzung</v-card-title>
  <v-card-text>
    <div ref="board" id="chessground-continuation"></div>
  </v-card-text>
</v-card>
</template>

<script lang="ts">
import { Chessground } from "chessground";
import type { Api } from "chessground/api";

import { loadPlayModel, type Model } from "@/neural-models/model";
import { completeOutputToMoves, fenToStandardPositionalInput } from '@/neural-models/chess_conversions';
import type { Key } from 'chessground/types';
import type { CompleteOutput, StandardPositionalInput } from '@/neural-models/types';
import { Chess } from 'chess.js';
import { loadSetting } from '@/settings/settings';

export default {
  props: {
    fen: {
      type: String,
      required: true,
    },
  },
  data: () => ({
    model: null as Model<StandardPositionalInput, CompleteOutput> | null,
    cg: null as Api | null,
    chess: null as Chess | null,
  }),
  created() {
    loadPlayModel(loadSetting("playModelName"))
      .then(model => this.model = model)
      .then(() => setTimeout(this.nextMove, 1500))
  },
  async mounted() {
    const config = {
      fen: this.fen,
      orientation: loadSetting("orientation"),
      viewOnly: true,
      coordinates: false,
      resizable: false,
      addPieceZIndex: true,
      highlight: {
        check: true,
      },
    } as const;

    this.cg = Chessground(this.$refs.board as HTMLElement, config);
    this.chess = new Chess(this.fen);
  },
  methods: {
    nextMove() {
      const input = fenToStandardPositionalInput(this.chess!.fen());
      const output = this.model!.predict(input);
      const moves = completeOutputToMoves(output);
      // Find the first legal move and play it.
      for (const move of moves) {
        if (this.chess!.move(move)) {
          this.cg!.move(move.from as Key, move.to as Key);
          break;
        }
      }
      if (this.chess!.isGameOver()) {
        return;
      }
      setTimeout(this.nextMove, 500);
    }
  }
}
</script>

<style scoped>
#chessground-continuation {
  width: 300px;
  height: 300px;
}
</style>
