<template>
  <div>
    <div id="board"></div>
  </div>
</template>

<script lang="ts">
import type { Model } from '@/neural-models/model'
import type { FromToOutput, StandardPositionalInput } from '@/neural-models/types'

import { loadFromToOutputModel } from '@/neural-models/model'
import { fromToOutputToMoves } from '@/neural-models/chess_conversions'

export default {
  created() {
    loadFromToOutputModel().then(m => this.model = m)
  },
  mounted() {
    // @ts-ignore
    window.board = new Chessboard('#board', {
      draggable: true,
      dropOffBoard: "trash",
      position: "start",
      pieceTheme: this.pieceTheme,
      // onDrop,
      // onSnapEnd,
    })
  },
  methods: {
    pieceTheme(piece: string) {
      const theme = "maestro";
      return `assets/img/chesspieces/${theme}/${piece}.svg`;
    }
  },
  data: () => ({
    message: 'Hello World!',
    model: null as Model<StandardPositionalInput, FromToOutput> | null
  }),
}
</script>
