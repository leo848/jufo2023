<template>
  <v-card min-height="200px">
    <v-card-title class="text-h5">Settings</v-card-title>
    <v-card-text>
      <v-container>
        <v-row>
          <v-col cols="12">
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

export default {
  data: () => ({
    settings: loadSettings(),
  }),
  methods: {
    save() {
      saveSettings(this.settings);
      this.$emit('close');
    },
  },
}
</script>

<style>

</style>
