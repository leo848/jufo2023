<template>
  <v-card max-width="400px" v-if="displayPieces().length">
    <v-card-title>Materialunterschied</v-card-title>
    <v-card-text class="mt-n4">
      <v-row no-gutters>
        <v-col cols="2" v-for="piece in displayPieces()" :key="piece">
          <v-chip color="primary" dark class="mb-n12 mt-0 pt-0">
            {{ piece.count }}
          </v-chip>
          <v-img :src="loadPiece(piece.id, theme)" aspect-ratio="1" class="ma-0 pa-0 piece" />
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import type { PropType } from 'vue'
import type { Piece, Role } from 'chessground/types'

import { loadPiece } from '@/chess/loadPieces'
import { loadSetting } from '@/settings/settings'
import { Chess } from 'chess.js'

type DisplayPiece = {
  id: string,
  count: number,
};

export default {
  props: {
    fen: {
      type: String as PropType<string>,
      required: true,
    },
  },
  data: () => ({
    theme: loadSetting('theme'),
  }),
  methods: {
    loadPiece,
    pieceToString(piece: Piece) {
      let roleAbbr: string;
      switch (piece.role) {
        case 'pawn':
          roleAbbr = 'P';
          break;
        case 'knight':
          roleAbbr = 'N';
          break;
        case 'bishop':
          roleAbbr = 'B';
          break;
        case 'rook':
          roleAbbr = 'R';
          break;
        case 'queen':
          roleAbbr = 'Q';
          break;
        case 'king':
          roleAbbr = 'K';
          break;
      }
      return piece.color.charAt(0) + roleAbbr;
    },
    roleToNumber(role: string): number {
      return ["k", "q", "r", "b", "n", "p"].indexOf(role.toLowerCase());
    },
    pieces(): DisplayPiece[] {
      // Calculate the pieces from the FEN.
      const fenToPieces = (fen?: string): DisplayPiece[] => {
        return new Chess(fen).board().flat().reduce((acc: DisplayPiece[], square: { type: string, color: string } | null) => {
          if (!square) return acc;
          const id = `${square.color}${square.type.toUpperCase()}`;
          const existing = acc.find((piece) => piece.id[1] === id[1]);
          if (existing) {
            if (existing.id[0] === id[0]) {
              existing.count += 1;
            } else {
              existing.count -= 1;
            }
          } else {
            acc.push({ id, count: 1 });
          }
          return acc;
        }, []);
      }

      const currentPieces = fenToPieces(this.fen);
      const startPieces = fenToPieces();

      // Calculate the difference between the current pieces and the start pieces.
      let diff = currentPieces.map((piece) => {
        const startPiece = startPieces.find((start) => start.id === piece.id);
        if (startPiece) {
          return { ...piece, count: piece.count - startPiece.count };
        }
        return piece;
      });

      return diff.map((piece) => {
        if (piece.count < 0) {
          return {
            id: piece.id[0] === 'w' ? `b${piece.id[1]}` : `w${piece.id[1]}`,
            count: -piece.count,
          }
        }
        return piece;
      }).filter((piece) => piece.count > 0);
    },
    displayPieces(): DisplayPiece[] {
      return this.pieces().sort((a, b) => {
        if (a.id[0] === b.id[0]) {
          return this.roleToNumber(a.id[1]) - this.roleToNumber(b.id[1]);
        }
        return a.id[0].localeCompare(b.id[0]);
      });
    }
  }
}
</script>

<style scoped>
.piece {
  z-index: -1;
}
</style>
