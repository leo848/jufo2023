<template>
  <v-card>
    <v-card-title class="text-h5">Game over!</v-card-title>
    <v-card-text class="mt-4 mb-4">
        <p class="text-h2 mb-4 font-weight-bold" v-html="result" />
        <p class="mb-2" v-html="winPieceIcons" />
        <p class="text-h5 mb-4">Reason: {{ status.reason }}</p>
        <v-card color="brown" @click="showPgn = !showPgn">
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
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { game } from "@/chess/game";
import { getStatus } from "@/chess/status";
import { loadPiece } from '@/chess/loadPieces';

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
          return `${this.pieceIcon("wK")}`;
      } else if (this.status.result === "1-0") {
          return `${this.pieceIcon("bK")}`;
      } else {
        return `${this.pieceIcon("wK")}&emsp;${this.pieceIcon("bK")}`;
      }
    },
    pgn() {
      return this.game.pgn();
    }
  },
  methods: {
    pieceIcon(piece: string) {
      return `
        <img src="${loadPiece(piece)}" class="small-piece"/>
      `;
    }
  }
}
</script>

<style>
.small-piece {
  width: 4rem;
  height: 4rem;
  /* The image should be in the middle of the text */
  vertical-align: middle;
  animation: pulse 0.5s infinite;
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
</style>
