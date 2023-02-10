<template>
  <v-card max-width="400px">
    <v-card-title>Wähle ein Theme aus</v-card-title>
    <v-card-text>
      <v-row>
        <v-col cols="12">
          <v-switch v-model="animate" label="Figuren animieren"/>
        </v-col>
        <v-col cols="3" v-for="theme, index in themes" :key="theme">
          <v-tooltip :text="theme" location="top">
            <template v-slot:activator="{ props }">
              <v-card @click="select(theme)" class="ma-0 pa-0" :color="oldTheme == theme ? 'primary' : null">
                <v-img :src="loadPiece(piece(index), theme)" v-bind="props"/>
              </v-card>
            </template>
          </v-tooltip>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { themes, loadPiece, pieces, type PieceTheme, colors } from '@/chess/loadPieces';

export default {
  mounted() {
    this.interval = setInterval(() => {
      if (this.animate) this.ticks++;
    }, 100);
  },
  unmounted() {
    clearInterval(this.interval);
  },
  props: {
    oldTheme: String,
  },
  data: () => ({
    ticks: 0,
    interval: 0,
    animate: false,
  }),
  computed: {
    themes() {
      return themes.filter(theme => theme !== 'companion');
    }
  },
  methods: {
    loadPiece(piece: string, theme: PieceTheme) {
      return loadPiece(piece, theme);
    },
    select(theme: string) {
      this.$emit('select', theme);
    },
    piece(index: number) {
      if (!this.animate) return 'wN'
      const timespan = 500;
      const idx = Math.floor((this.ticks - index + timespan) % timespan / (timespan / 12));
      const piece = pieces[Math.floor(idx / 2)];
      const color = colors[idx % 2];
      return color + piece;
    }
  },
}
</script>
