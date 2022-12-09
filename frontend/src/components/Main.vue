<template>
  <div>
    <v-row>
      <v-col cols="12" sm="6" lg="4">
        <div ref="board"></div>
      </v-col>
      <v-col cols="12" sm="6" lg="4">
        <MoveDisplay v-if="model" :moves="moves" />
        <v-card v-else max-width="300px">
          <v-card-title>Loading model...</v-card-title>
          <v-card-text>
            <v-progress-circular :size="50" :width="6" indeterminate />
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import type { Model } from '@/neural-models/model'
import type { CompleteOutput, MoveWithAct, StandardPositionalInput } from '@/neural-models/types'

import { loadModel } from '@/neural-models/model'
import { completeOutputToMoves, fenToStandardPositionalInput } from '@/neural-models/chess_conversions'
import { game, addEvent, removeEvent, isSquare } from '@/chess/game'
import { getMove } from '@/chess/boardHandlers'
import { loadPiece } from '@/chess/loadPieces';

import MoveDisplay from '@/components/MoveDisplay.vue'
import type { Chess, Move } from 'chess.js'
import { loadSetting } from '@/settings/settings'

export default {
  components: {
    MoveDisplay,
  },
  created() {
    loadModel("3m-unique-rust-model").then(m => this.model = m).then(this.update)
  },
  data: () => ({
    message: 'Hello World!',
    event: 0,
    model: null as Model<StandardPositionalInput, CompleteOutput> | null,
    moves: [] as MoveWithAct[],
    board: null as any | null,
  }),
  mounted() {
    this.board = (window["Chessboard" as any] as any)(this.$refs.board, {
      draggable: true,
      dropOffBoard: "trash",
      position: "start",
      pieceTheme: this.pieceTheme,
      onDrop: this.onDrop,
      onDragStart: this.onDragStart,
      onSnapEnd: this.onSnapEnd,
    })
    this.event = addEvent((game: Chess) => {
      this.board.position(game.fen())
      this.update();
    });

    (this.$refs.board as HTMLElement).addEventListener('scroll touchmove touchend touchstart contextmenu', (e) => {
      alert(e);
      e.preventDefault()
    });
  },
  beforeUnmount() {
    removeEvent(this.event);
  },
  methods: {
    pieceTheme(piece: string) {
      return loadPiece(piece);
    },
    onDragStart(source: string, piece: string, _position: string, _orientation: string) {
      if (!isSquare(source)) throw new Error("source isn't square");
      if (game.isGameOver()) return false;
      if (game.turn() === "w" && piece.search(/^b/) !== -1) return false;
      if (game.turn() === "b" && piece.search(/^w/) !== -1) return false;
      if (game.moves({ square: source }).length === 0) return false;
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
      let moves: (MoveWithAct & {inner : null | Move, index: number })[] = [];
      const onlyShowLegalMoves = loadSetting("onlyShowLegalMoves");
      while (moves.length < 8 && amount <= 10000) {
        moves = completeOutputToMoves(output, { amount })
          .map((obj, index) => ({ ...obj, inner: getMove(obj), index: index + 1 })) 
          .filter(obj => !onlyShowLegalMoves || obj.inner !== null)
          .slice(0, 8) ;
        amount *= 10;
      }
      this.moves = moves;
    }
  },
}
</script>
