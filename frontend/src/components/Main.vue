<template>
  <div>
    <div id="board"></div>
    <MoveDisplay :moves="moves" />
  </div>
</template>

<script lang="ts">
import type { Model } from '@/neural-models/model'
import type { FromToOutput, MoveWithAct, StandardPositionalInput } from '@/neural-models/types'

import { loadFromToOutputModel } from '@/neural-models/model'
import { fenToStandardPositionalInput, fromToOutputToMoves } from '@/neural-models/chess_conversions'
import { game } from '@/chess/game'
import type { Chessboard } from '@/chess/boardHandlers'

import MoveDisplay from '@/components/MoveDisplay.vue'

export default {
  components: {
    MoveDisplay,
  },
  created() {
    loadFromToOutputModel().then(m => this.model = m)
  },
  mounted() {
    this.board = Chessboard('#board', {
      draggable: true,
      dropOffBoard: "trash",
      position: "start",
      pieceTheme: this.pieceTheme,
      onDrop: this.onDrop,
      onSnapEnd: this.onSnapEnd,
    })
  },
  data: () => ({
    message: 'Hello World!',
    model: null as Model<StandardPositionalInput, FromToOutput> | null,
    moves: [] as MoveWithAct[],
    board: null as any | null,
  }),
  methods: {
    pieceTheme(piece: string) {
      const theme = "cardinal";
      return `assets/img/chesspieces/${theme}/${piece}.svg`;
    },
    onDrop(source: string, target: string) {
      let localGame = game;
      let move = localGame.move({
        from: source,
        to: target,
      });


      if (move === null) return "snapback";

      this.update();
    },
    onSnapEnd() {
      this.board.position(game.fen());
    },
    update() {
      if (this.model === null) return;
      const input = game.fen();
      const output = this.model.predict(fenToStandardPositionalInput(input));
      const moves = fromToOutputToMoves(output);
      this.moves = moves;
    }
  },
}
</script>

<style>
#board {
  width: 400px;
  height: 400px;
}
</style>
