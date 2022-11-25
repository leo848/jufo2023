<template>
  <v-card min-height="200px">
    <v-card-title class="text-h5">Settings</v-card-title>
    <v-card-text>
      <v-container>
        <v-row>
          <v-col cols="12">
            <v-menu v-model="pieceThemeMenu" :close-on-content-click="false">
              <template v-slot:activator="{ props }">
                <v-btn v-bind="props" class="mr-4" color="primary">{{ settings.theme }}</v-btn>
              </template>
              <ChooseTheme @select="selectTheme"/>
            </v-menu>
            <span>by {{ license.by }} <span v-if="license.license">under {{ license.license }}</span></span>
          </v-col>
          <v-col cols="12" md="6">
            <v-switch
              v-model="settings.onlyShowLegalMoves"
              color="primary"
              label="Only show legal moves"
              />
          </v-col>
        </v-row>
      </v-container>
    </v-card-text>
    <v-spacer />
    <v-card-actions>
      <v-spacer />
      <v-btn @click="$emit('close')">Cancel</v-btn>
      <v-btn @click="save" variant="tonal" color="primary">Save</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import { loadSettings, saveSettings } from '@/settings/settings';
import { themes, themesLicenses, type PieceTheme } from '@/chess/loadPieces';

import ChooseTheme from '@/components/ChooseTheme.vue';

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
    }
  },
  methods: {
    save() {
      saveSettings(this.settings);
      this.$emit('close');
    },
    selectTheme(theme: PieceTheme) {
      this.settings.theme = theme;
      this.pieceThemeMenu = false;
    },
  },
}
</script>
