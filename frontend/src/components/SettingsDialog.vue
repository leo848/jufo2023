<template>
  <v-card min-height="200px">
    <v-card-title class="text-h3 pt-6 pb-2">Einstellungen</v-card-title>
    <v-card-text>
      <v-container>
        <v-row>
          <v-col cols="12">
            <v-menu v-model="pieceThemeMenu" :close-on-content-click="false">
              <template v-slot:activator="{ props }">
                <v-btn v-bind="props" class="mr-4" color="primary" icon size="x-large">
                  <img :src="loadPiece('wN')" class="button-piece" />
                </v-btn>
              </template>
              <ChooseTheme @select="selectTheme" :oldTheme="settings.theme" />
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
              :disabled="!settings.show.neuralOutput"
              v-model="settings.onlyShowLegalMoves"
              color="primary"
              label="Nur legale Züge anzeigen"
              density="compact"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-switch
              :disabled="!settings.show.neuralOutput"
              v-model="settings.showActivation"
              color="primary"
              label="Neuronale Aktivierung zeigen"
              density="compact"
            />
          </v-col>
          <v-col cols="12">
            <v-slider
              :disabled="!settings.show.neuralOutput"
              v-model="settings.maxMoves"
              :min="1"
              :max="20"
              :step="1"
              thumb-label
              label="Maximale angezeigte Züge"
            >
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
          <v-col cols="12" md="5">
            <p class="mb-2">Automatisch spielen für:</p>

            <v-btn
              @click="settings.autoPlay.white = !settings.autoPlay.white"
              :color="settings.autoPlay.white ? 'primary' : null"
              class="mr-4"
              icon
              size="x-large"
            >
              <img :src="loadPiece('wK')" class="button-piece" />
            </v-btn>
            <v-btn
              @click="settings.autoPlay.black = !settings.autoPlay.black"
              :color="settings.autoPlay.black ? 'primary' : null"
              icon
              size="x-large"
            >
              <img :src="loadPiece('bK')" class="button-piece" />
            </v-btn>
          </v-col>
          <v-col cols="12" md="7">
            <p class="mb-2">Neuronale Elemente anzeigen:</p>
            <span
              v-for="[element, deutsch, desc] in [
                ['neuralOutput', 'ausgabe', 'Zeige die Züge an, die das neuronale Netzwerk in der aktuellen Situation spielen würde.'],
                ['capturedPieces', 'material', 'Zeige den Materialunterschied an – welche Seite mehr Figuren hat'],
                ['evaluation', 'evaluierung', 'Ein weiteres neuronales Netzwerk zeigt die aktuelle Bewertung des Spielfelds an.'],
                ['continuation', 'fortsetzung', 'Wie würde das aktuell ausgewählte neuronale Netzwerk diese Partie für beide Seiten weiter spielen?'],
              ]"
              :key="element"
            >
              <v-btn
                :color="settings.show[element] ? 'primary' : null"
                class="mr-4 mb-4"
              >
              <span
                @click="settings.show[element] = !settings.show[element]"
                >
                {{ deutsch }}
              </span>
                <span class="ml-3">
                  <v-tooltip :text="desc" max-width="250px">
                    <template v-slot:activator="{ props }">
                      <v-icon v-bind="props">mdi-information</v-icon>
                    </template>
                  </v-tooltip>
                </span>
              </v-btn>
            </span>
          </v-col>
          <v-col cols="12">
            <v-slider
              v-model="settings.temperature"
              :min="0"
              :max="1"
              :step="0.025"
              thumb-label
              label="Temperatur"
            >
              <template v-slot:append>
                <v-text-field
                  v-model="settings.temperature"
                  hide-details
                  single-line
                  disabled
                  density="compact"
                  type="number"
                  style="width: 90px"
                ></v-text-field>
              </template>
            </v-slider>
          </v-col>
        </v-row>
      </v-container>
    </v-card-text>
    <v-spacer />
    <v-card-actions>
      <v-spacer />
      <v-btn @click="$emit('cancel')">Abbrechen</v-btn>
      <v-btn @click="save" variant="tonal" color="primary">Speichern</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import { loadSettings, saveSettings } from "@/settings/settings";
import {
  loadPiece,
  themes,
  themesLicenses,
  type PieceTheme,
} from "@/chess/loadPieces";

import ChooseTheme from "@/components/ChooseTheme.vue";

export default {
  components: { ChooseTheme },
  emits: ["cancel", "save"],
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
      this.$emit("save");
    },
    selectTheme(theme: PieceTheme) {
      this.settings.theme = theme;
      this.pieceThemeMenu = false;
    },
    loadPiece(piece: string) {
      return loadPiece(piece, this.settings.theme);
    },
  },
};
</script>

<style>
.button-piece {
  height: 80%;
  width: 80%;
}
</style>
