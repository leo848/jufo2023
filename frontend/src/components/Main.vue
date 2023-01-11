<template>
  <div>
    <v-row>
      <v-col cols="12" sm="6" lg="4">
        <div ref="board" id="chessground-main"></div>
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

import { Chessground } from "chessground";
import type { Api } from "chessground/api";
import type { Key, Role } from "chessground/types";

export default {
  components: {
    MoveDisplay,
    GameOver,
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
    board: null as Api | null,
    autoPlay: loadSetting("autoPlay"),
    gameOver: false,
  }),
  mounted() {
    // The chess start fen, expressed in Forsyth–Edwards Notation
    const config = {
      autoCastle: true,
      movable: {
        free: false,
      },
      events: {
        move: (orig: Key, dest: Key) => {
          if (this.gameOver) return;
          const move = getMove({ from: orig, to: dest });
          if (move) {
            game.move(move);
            this.update();
          }
        },
      },
      highlight: {
        lastMove: true,
        check: true,
      },
      animation: {
        enabled: true,
        duration: 500,
      },
    };

    if (!(this.$refs.board instanceof HTMLElement))
      throw new Error("Board ref is not an HTMLElement");

    this.board = Chessground(this.$refs.board, config);

    this.event = addEvent((game: Chess) => {
      this.board!.set({
        fen: game.fen(),
      });
      this.update();
    });

    // Create a DOM <style> element, styling every chess piece with the setting

    const style = document.createElement("style");
    style.id = "pieceStyle";
    const pieceTheme = loadSetting("theme");

    // For every role in the type Role:

    for (const [role, shortRole] of [["pawn", "P"], ["knight", "N"], ["bishop", "B"], ["rook", "R"], ["queen", "Q"], ["king", "K"]]) {
      for (const [color, shortColor] of [["white", "w"], ["black", "b"]]) {
        const piece = loadPiece(`${shortColor}${shortRole}`, pieceTheme);
        style.innerHTML += `.cg-wrap piece.${role}.${color} { background-image: url("${piece}"); }`;
      }
    }

    document.head.appendChild(style);
  },
  beforeUnmount() {
    removeEvent(this.event);

    const style = document.getElementById("pieceStyle");
    if (style) style.remove();
  },
  methods: {
    pieceTheme(piece: string) {
      return loadPiece(piece);
    },
    update() {
      const destinations: Map<Key, Key[]> = new Map();
      for (const move of game.moves({ verbose: true })) {
        if (typeof move === "string") throw new Error("move is string");
        if (destinations.get(move.from as Key) === undefined) {
          destinations.set(move.from as Key, []);
        }
        destinations.get(move.from as Key)!.push(move.to as Key);
      }

      const newConfig = {
        fen: game.fen(),
        movable: {
          free: false,
          dests: destinations,
        },
      };

      this.board!.set(newConfig);

      this.updateMoves();

      if (game.isGameOver())
        setTimeout(() => {
          this.gameOver = true;
        }, 100);
    },

    updateMoves() {
      if (this.model === null) return;
      const input = game.fen();
      const output = this.model.predict(fenToStandardPositionalInput(input));

      const { maxMoves, onlyShowLegalMoves } = loadSettings();

      let amount = 10;
      let moves: (MoveWithAct & { inner: null | Move; index: number })[] = [];
      while (moves.length < maxMoves && amount <= 10000) {
        moves = completeOutputToMoves(output)
          .slice(0, amount)
          .map((obj, index) => ({
            ...obj,
            inner: getMove(obj),
            index: index + 1,
          }))
          .filter((obj) => !onlyShowLegalMoves || obj.inner !== null);
        amount *= 10;
      }

      const currentColor = game.turn();
      if (
        !game.isGameOver() &&
        ((this.autoPlay.black && currentColor === "b") ||
          (this.autoPlay.white && currentColor === "w"))
      ) {
        setTimeout(() => {
          let move = null;
          let counter = 0;
          while (move?.inner == null) {
            move = moves[counter++];
          }
          if (move.inner !== null) {
            game.move(move.inner);
            this.board!.move(move.inner.from as Key, move.inner.to as Key);
            this.update();
          }
        }, 100);
      } else {
        this.moves = moves;
      }
    },

    newGame() {
      this.gameOver = false;
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

#chessground-main {
  width: 500px;
  height: 500px;
  position: relative;
  overflow: hidden;
}

cg-board {
  background-color: #bfcfdd;
}
</style>
