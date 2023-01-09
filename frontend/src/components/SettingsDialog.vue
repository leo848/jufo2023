<template>
  <v-card min-height="200px">
    <v-card-title class="text-h5">Einstellungen</v-card-title>
    <v-card-text>
      <v-container>
        <v-row>
          <v-col cols="12">
            <v-menu v-model="pieceThemeMenu" :close-on-content-click="false">
              <template v-slot:activator="{ props }">
                <v-btn v-bind="props" class="mr-4" color="primary">{{
                  settings.theme
                }}</v-btn>
              </template>
              <ChooseTheme @select="selectTheme" />
            </v-menu>
            <span
              >von {{ license.by }}
              <span v-if="license.license"
                >, lizenziert unter {{ license.license }}</span
              ></span
            >
          </v-col>
          <v-col cols="12" md="6">
            <v-switch
              v-model="settings.onlyShowLegalMoves"
              color="primary"
              label="Nur legale Züge anzeigen"
              density="compact"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-switch
              v-model="settings.showActivation"
              color="primary"
              label="Neuronale Aktivierung zeigen"
              density="compact"
            />
          </v-col>
          <v-col cols="12">
            <v-slider v-model="settings.maxMoves" :min="1" :max="20" :step="1" thumb-label label="Maximale angezeigte Züge">
              <template v-slot:append>
                <v-text-field
                  v-model="settings.maxMoves"
                  hide-details
                  single-line
                  density="compact"
                  type="number"
                  style="width: 70px"
                  ></v-text-field>
              </template>
            </v-slider>
          </v-col>
          <v-col cols="12" md="6">
            <p class="mb-2">Automatisch spielen für:</p>

            <v-btn @click="settings.autoPlay.white = !settings.autoPlay.white" :color="settings.autoPlay.white ? 'primary' : null" class="mr-4" icon size="x-large">
              <img :src="loadPiece('wK')" class="button-piece"/>
            </v-btn>
            <v-btn @click="settings.autoPlay.black = !settings.autoPlay.black" :color="settings.autoPlay.black ? 'primary' : null" icon size="x-large">
              <img :src="loadPiece('bK')" class="button-piece"/>
            </v-btn>
          </v-col>
        </v-row>
      </v-container>
    </v-card-text>
    <v-spacer />
    <v-card-actions>
      <v-spacer />
      <v-btn @click="$emit('close')">Abbrechen</v-btn>
      <v-btn @click="save" variant="tonal" color="primary">Speichern</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import { loadSettings, saveSettings } from "@/settings/settings";
import { loadPiece, themes, themesLicenses, type PieceTheme } from "@/chess/loadPieces";

import ChooseTheme from "@/components/ChooseTheme.vue";

export default {
  components: { ChooseTheme },
  data: () => ({
    settings: loadSettings(),
    themes: themes,
    pieceThemeMenu: false,
  }),
  computed: {
    license() {
      return themesLicenses[this.settings.theme];
    },
  },
  methods: {
    save() {
      saveSettings(this.settings);
      this.$emit("close");
    },
    selectTheme(theme: PieceTheme) {
      this.settings.theme = theme;
      this.pieceThemeMenu = false;
    },
    loadPiece,
  },
};
</script>

<style>
.button-piece {
  height: 80%;
  width: 80%;
}
</style>
