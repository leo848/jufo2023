<template>
  <div>
    <v-row>
      <v-col cols="12" sm="6" lg="4">
        <div id="board"></div>
      </v-col>
      <v-col cols="12" sm="6" lg="4">
        <MoveDisplay :moves="moves" />
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import type { Model } from '@/neural-models/model'
import type { FromToOutput, MoveWithAct, StandardPositionalInput } from '@/neural-models/types'

import { loadFromToOutputModel } from '@/neural-models/model'
import { fenToStandardPositionalInput, fromToOutputToMoves } from '@/neural-models/chess_conversions'
import { game, addEvent } from '@/chess/game'
import type { Chessboard } from '@/chess/boardHandlers'
import { getMove } from '@/chess/boardHandlers'
import { loadPiece } from '@/chess/loadPieces';

import MoveDisplay from '@/components/MoveDisplay.vue'
import type { Chess, Move } from 'chess.js'

export default {
  components: {
    MoveDisplay,
  },
  created() {
    loadFromToOutputModel().then(m => this.model = m).then(this.update)
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
    addEvent((game: Chess) => {
      this.board.position(game.fen())
      this.update();
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
      return loadPiece(piece);
    },
    onDrop(source: string, target: string) {
      let move = game.move({
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
      let amount = 10;
      let moves: (MoveWithAct & {inner : null | Move })[] = [];
      while (moves.length < 8 && amount <= 10000) {
        moves = fromToOutputToMoves(output, { amount })
          .map(obj => ({ ...obj, inner: getMove(obj) })) 
          .filter(obj => obj.inner !== null)
          .slice(0, 8) ;
        amount *= 10;
      }
      this.moves = moves;
    }
  },
}
</script>
