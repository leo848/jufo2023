<template>
  <v-card>
    <v-card-title>Autoenkodierung</v-card-title>
    <v-card-text>
      <div ref="boardauto" id="chessground-autoencoder"></div>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { standardPositionalInputToFen, fenToStandardPositionalInput } from '@/neural-models/chess_conversions';
import { type Model, loadAutoencoderModel } from '@/neural-models/model'
import type { StandardPositionalInput as SpiType } from '@/neural-models/types';
import { loadSetting } from '@/settings/settings';
import type { Api as ChessgroundApi } from 'chessground/api';
import { Chessground } from 'chessground';

export default {
  name: 'Autoencoder',
  data: () => ({
    model: null as Model<SpiType, SpiType> | null,
    cg: null as ChessgroundApi | null,
  }),
  props: {
    fen: {
      type: String,
      required: true,
    },
  },
  created() {
    loadAutoencoderModel(loadSetting("autoencoderModelName"))
      .then(model => {
        this.model = model;
        this.update();
      })
  },
  mounted() {
    const config = {
      viewOnly: true,
      coordinates: true,
      resizable: false,
      addPieceZIndex: true,
      highlight: {
        check: true,
      },
    } as const;

    this.cg = Chessground(this.$refs.boardauto as HTMLElement, config);
  },
  watch: {
    fen() {
      this.update();
    }
  },
  methods: {
    update() {
      if (!this.model) {
        return;
      }
      const positionalInput = fenToStandardPositionalInput(this.fen);
      const position = this.model.predict(positionalInput);
      const decodedPosition = standardPositionalInputToFen(position);
      this.cg!.set({ fen: decodedPosition });
    }
  }
}
</script>

<style scoped>
#chessground-autoencoder {
  width: 300px;
  height: 300px;
}
</style>
