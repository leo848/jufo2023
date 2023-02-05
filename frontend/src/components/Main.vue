<template>
  <div>
    <v-row>
      <v-col cols="12" sm="8" md="6" lg="4">
        <div ref="board" id="chessground-main"></div>
        <Evaluation :fen="fen" v-if="!gameOver" class="mt-4"/>
        <CapturedPieces v-if="show.capturedPieces && !gameOver" :fen="fen" class="mt-2"/>
      </v-col>
      <v-col cols="12" sm="4" lg="3">
        <div v-if="gameOver">
          <GameOver @new="newGame" />
        </div>
        <div v-else>
          <MoveDisplay v-if="show.neuralOutput && model && !gameOver" :moves="moves" :legals="legals" @suggest="suggestMove" @showAll="showAll" />
          <v-card v-else-if="show.neuralOutput && !gameOver" max-width="300px">
            <v-card-title>Lade Modell...</v-card-title>
            <v-card-text>
              <v-progress-circular :size="50" :width="6" indeterminate />
            </v-card-text>
          </v-card>
        </div>
      </v-col>
      <v-col cols="12" sm="4" lg="3">
        <div v-if="show.continuation && model && !gameOver">
          <Continuation :fen="fen" :key="fen"/>
        </div>
      </v-col>
    </v-row>
  </div>
</template>

<script lang="ts">
import MoveDisplay from "@/components/MoveDisplay.vue";
import GameOver from "@/components/GameOver.vue";
import Continuation from "@/components/Continuation.vue";
import Evaluation from "@/components/Evaluation.vue";
import CapturedPieces from "@/components/CapturedPieces.vue";

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
import { game, addEvent, removeEvent } from "@/chess/game";
import { getMove } from "@/chess/boardHandlers";
import { loadPiece } from "@/chess/loadPieces";

import type { Chess, Move } from "chess.js";
import type { Piece as ChessgroundPiece } from "chessground/types";
import { loadSetting, loadSettings } from "@/settings/settings";

import { Chessground } from "chessground";
import type { Api } from "chessground/api";
import type { Key } from "chessground/types";
import { temperature } from '@/neural-models/temperature';

export default {
  components: {
    MoveDisplay,
    GameOver,
    Continuation,
    Evaluation,
    CapturedPieces,
  },
  created() {
    loadModel(loadSetting("model"))
      .then((m) => (this.model = m))
      .then(this.update);
  },
  data: () => ({
    message: "Hello World!",
    event: 0,
    model: null as Model<StandardPositionalInput, CompleteOutput> | null,
    moves: [] as MoveWithAct[],
    legals: -1,
    board: null as Api | null,
    capturedPieces: [] as ChessgroundPiece[],
    autoPlay: loadSetting("autoPlay"),
    show: loadSetting("show"),
    gameOver: false,
    fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
  }),
  mounted() {
    const config = {
      autoCastle: true,
      movable: {
        free: false,
      },
      events: {
        move: (orig: Key, dest: Key, capturedPiece: ChessgroundPiece | undefined) => {
          if (this.gameOver) return;
          const move = getMove({ from: orig, to: dest });
          if (move) {
            game.move(move);
            this.update();
          }
          if (capturedPiece) {
            this.capturedPieces.push(capturedPiece);
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

      this.fen = game.fen();

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

      let amount = 500;
      let moves: (MoveWithAct & { inner: null | Move; index: number })[] = [];
      while (moves.length < maxMoves && amount <= 10000) {
        moves = completeOutputToMoves(output)
          .slice(0, amount)
          .map((obj, index) => ({
            ...obj,
            inner: getMove(obj),
            index: index + 1,
          }))
        amount *= 10;
      }

      const currentColor = game.turn();
      if (
        !game.isGameOver() &&
        ((this.autoPlay.black && currentColor === "b") ||
          (this.autoPlay.white && currentColor === "w"))
      ) {
        setTimeout(() => {
          let probs = this.assignProbabilities(moves);
          let move = this.chooseMove(probs);
          this.board!.move(move.from as Key, move.to as Key);
          this.update();
        }, 100);
      } else {
        this.moves = moves;
      }

      const legalMoves = moves.filter((move) => move.inner !== null);
      const activationSum = legalMoves.reduce((acc, move) => acc + move.act, 0);

      this.legals = activationSum;

    },

    assignProbabilities(moves: MoveWithAct[]): (MoveWithAct & { prob: number })[] {
      const temp = loadSetting("temperature");
      const probabilities = temperature(moves.map((move) => move.act), temp);
      return moves.map((move, i) => {
        return Object.assign({}, move, { prob: probabilities[i] });
      });
    },

    chooseMove(moves: (MoveWithAct & { prob: number })[], tries: number = 20): MoveWithAct {
      if (tries <= 0) return this.fallbackChooseMove(moves);
      let rng = Math.random();
      let chosen = null;

      moves = moves.filter(move => getMove(move) != null);
      const sum = moves.reduce((acc, move) => acc + move.prob, 0);
      moves = moves.map(move => Object.assign({}, move, { prob: move.prob / sum })); // Normalize

      for (const move of moves) {
        if (rng < move.prob) {
          chosen = move;
          break;
        } else {
          rng -= move.prob;
        }
      }
      if (chosen == null) {
        return this.chooseMove(moves, tries-1);
      }
      else if (game.move(chosen) == null) {
        return this.chooseMove(moves, tries-0.1);
      }
      return chosen;
    },

    fallbackChooseMove(moves: MoveWithAct[]): MoveWithAct {
      console.log("fallback");
      return moves.filter(move => getMove(move) != null)[0]
    },

    moveToShape(move: MoveWithAct, index ?: number) {
      let brush = index === 0 ? "blue" : "paleBlue";
      if (getMove(move) == null) {
        brush = index === 0 ? "red" : "paleRed";
      }
      const prepareLineWidth = (n: number) => {
        return 25 * Math.pow(n, 2 / 5) + 1;
      }
      return {
        orig: move.from as Key,
        dest: move.to as Key,
        brush,
        modifiers: {
          lineWidth: prepareLineWidth(move.act),
        }
      };
    },

    suggestMove(move?: MoveWithAct) {
      if (!move) {
        this.board!.setShapes([]);
        return;
      }
      this.board!.setShapes([ this.moveToShape(move) ]); 
    },

    showAll() {
      this.board!.setShapes(
        this.moves
          .filter(move => move.act > 0.01)
          .map(this.moveToShape)
      );
    },

    newGame() {
      this.gameOver = false;
      // Remove the remnants of the last move, i.e. the last move's
      // destination squares.
      this.board!.set({
        lastMove: undefined,
      });
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
  width: 100%;
  aspect-ratio: 1 / 1;
  position: relative;
  overflow: hidden;
}

cg-board {
  background-color: #bfcfdd;
}
</style>
