<template>
  <v-card>
    <v-card-title class="text-h5">{{ gameOverString() }}</v-card-title>
    <v-card-text class="mt-4 mb-4">
      <p class="text-h2 mb-4 font-weight-bold" v-html="result" />
      <p class="mb-2" v-html="winPieceIcons" />
      <p class="text-h5 mb-4">Grund: {{ translateReason(status.reason) }}</p>
      <v-card :color="showPgn ? null : 'brown'" @click="showPgn = !showPgn">
        <v-card-title>PGN</v-card-title>
        <v-slide-y-transition>
          <v-card-text v-show="showPgn">
            <v-textarea
              v-model="pgn"
              :rows="1"
              :auto-grow="true"
              :counter="false"
              :outlined="true"
              :readonly="true"
              @click.stop
            />
          </v-card-text>
        </v-slide-y-transition>
      </v-card>
      <v-btn size="x-large" class="mt-6 rainbow-btn" @click="newGame">Neues Spiel</v-btn>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { game, resetGame } from "@/chess/game";
import { getStatus, translateReason } from "@/chess/status";
import { loadPiece } from "@/chess/loadPieces";

export default {
  data: () => ({
    game,
    status: getStatus(game),
    showPgn: false,
  }),
  computed: {
    result() {
      if (this.status.result === "1-0") {
        return "1 - 0";
      } else if (this.status.result === "0-1") {
        return "0 - 1";
      } else {
        return "½ - ½";
      }
    },
    winPieceIcons() {
      if (this.status.result === "0-1") {
        return `${this.pieceIcon("bK")}`;
      } else if (this.status.result === "1-0") {
        return `${this.pieceIcon("wK")}`;
      } else {
        return `${this.pieceIcon("wK")}&emsp;${this.pieceIcon("bK")}`;
      }
    },
    pgn() {
      return this.game.pgn();
    },
  },
  methods: {
    pieceIcon(piece: string) {
      return `
        <img src="${loadPiece(piece)}" class="small-piece"/>
      `;
    },
    newGame() {
      resetGame();
      this.$emit("new");
    },
    gameOverString() {
      const strings = ["Game over", "Spiel vorbei!"];
      if (this.status.result === "0-1") {
        strings.push("Schwarz gewinnt!", "Sieg für Schwarz!");
      } else if (this.status.result === "1-0") {
        strings.push("Weiß gewinnt!", "Sieg für Weiß!");
      } else {
        strings.push("Unentschieden!", "Remis!");
      }
      return strings[Math.random() * strings.length | 0]
    },
    translateReason,
  },
  emits: [ "new" ]
};
</script>

<style>
.small-piece {
  width: 4rem;
  height: 4rem;
  /* The image should be in the middle of the text */
  vertical-align: middle;
  animation: pulse 0.5s;
}

@keyframes pulse {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.02);
  }
  100% {
    transform: scale(1);
  }
}

.rainbow-btn {
  background: linear-gradient(
    to right,
    #e40303,
    #ff8c00,
    #ffed00,
    #008026,
    #24408e,
    #732982,
    #e40303
  );
  background-size: 600% 400%;
  animation: move-bg 120s linear infinite;
  transition: all 0.2s;
}

.rainbow-btn:hover {
  filter: invert(100%) contrast(200%);
}

@keyframes move-bg {
  0% {
    background-position: 0 0;
  }
  100% {
    background-position: 400% 0;
  }
}
</style>
