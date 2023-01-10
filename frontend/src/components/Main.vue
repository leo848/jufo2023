<template>
  <div>
    <v-row>
      <v-col cols="12" sm="6" lg="4">
        <div ref="board"></div>
      </v-col>
      <v-col cols="12" sm="6" lg="4">
        <div v-if="gameOver">
          <GameOver @new="newGame" />
        </div>
        <div v-else>
          <MoveDisplay v-if="model && !gameOver" :moves="moves" />
          <v-card v-else-if="!gameOver" max-width="300px">
            <v-card-title>Loading model...</v-card-title>
            <v-card-text>
              <v-progress-circular :size="50" :width="6" indeterminate />
            </v-card-text>
          </v-card>
        </div>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import type { Model } from "@/neural-models/model";
import type {
  CompleteOutput,
  MoveWithAct,
  StandardPositionalInput,
} from "@/neural-models/types";

import { loadModel } from "@/neural-models/model";
import {
  completeOutputToMoves,
  fenToStandardPositionalInput,
} from "@/neural-models/chess_conversions";
import { game, addEvent, removeEvent, isSquare } from "@/chess/game";
import { getMove } from "@/chess/boardHandlers";
import { loadPiece } from "@/chess/loadPieces";

import MoveDisplay from "@/components/MoveDisplay.vue";
import GameOver from "@/components/GameOver.vue";
import type { Chess, Move } from "chess.js";
import { loadSetting, loadSettings } from "@/settings/settings";

export default {
  components: {
    MoveDisplay, GameOver,
  },
  created() {
    loadModel("15mtrain-512neurons-4layers")
      .then((m) => (this.model = m))
      .then(this.update);
  },
  data: () => ({
    message: "Hello World!",
    event: 0,
    model: null as Model<StandardPositionalInput, CompleteOutput> | null,
    moves: [] as MoveWithAct[],
    board: null as any | null,
    autoPlay: loadSetting("autoPlay"),
    gameOver: false,
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
    });
    this.event = addEvent((game: Chess) => {
      this.board.position(game.fen());
      this.update();
    });

    (this.$refs.board as HTMLElement).addEventListener(
      "scroll touchmove touchend touchstart contextmenu",
      (e) => {
        alert(e);
        e.preventDefault();
      }
    );
  },
  beforeUnmount() {
    removeEvent(this.event);
  },
  methods: {
    pieceTheme(piece: string) {
      return loadPiece(piece);
    },
    onDragStart(
      source: string,
      piece: string,
      _position: string,
      _orientation: string
    ) {
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

      const { maxMoves, onlyShowLegalMoves } = loadSettings();

      let amount = 10;
      let moves: (MoveWithAct & { inner: null | Move; index: number })[] = [];
      while (moves.length < maxMoves && amount <= 10000) {
        moves = completeOutputToMoves(output).slice(0, amount)
          .map((obj, index) => ({
            ...obj,
            inner: getMove(obj),
            index: index + 1,
          }))
          .filter((obj) => !onlyShowLegalMoves || obj.inner !== null);
        amount *= 10;
      }
      this.moves = moves;

      const currentColor = game.turn();
      if (
        !game.isGameOver() &&
        ((this.autoPlay.black && currentColor === "b") ||
          (this.autoPlay.white && currentColor === "w"))
      ) {
        requestAnimationFrame(() => {
          let move = null;
          let counter = 0;
          while (move?.inner == null) {
            move = moves[counter++];
          }
          if (move.inner !== null) {
            game.move(move.inner);
            this.board.position(game.fen());
            this.update();
          }
        })
      }
      if (game.isGameOver()) {
        this.gameOver = true;
        console.log("Game over!");
      }
    },
    newGame() {
      this.gameOver = false;
    }
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
