<template>
  <v-card min-height="200px">
    <v-card-title class="text-h5">Settings</v-card-title>
    <v-card-text>
      <v-container>
        <v-row>
          <v-col cols="12" md="6">
            <v-select v-model="settings.theme" :items="themes" density="comfortable" label="Piece theme"/> <!-- TODO: custom menu -->
                      <span>by {{ license.by }} <span v-if="license.license">under {{ license.license }}</span></span>
          </v-col>
          <v-col cols="0" md="6"></v-col>
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
import { themes, themesLicenses } from '@/chess/loadPieces';

export default {
  data: () => ({
    settings: loadSettings(),
    themes: themes,
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
  },
}
</script>
