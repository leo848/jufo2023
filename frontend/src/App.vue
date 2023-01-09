<template>
  <v-app>
    <v-app-bar ref="app-bar" app floating>
      <v-app-bar-title>Kann Künstliche Intelligenz Schach lernen?</v-app-bar-title>
      <v-spacer></v-spacer>
      <v-btn icon>
        <v-icon>mdi-cog</v-icon>
        <v-dialog
          v-model="dialog"
          activator="parent"
          max-width="800px"
          >
          <SettingsDialog @close="newSettings" />
        </v-dialog>
      </v-btn>
    </v-app-bar>
    <v-main class="mt-4" :key="mainKey">
      <router-view/>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import SettingsDialog from '@/components/SettingsDialog.vue'

export default {
  name: 'App',
  components: { SettingsDialog },
  data: () => ({
    dialog: false,
    mainKey: 0,
  }),
  methods: {
    newSettings() {
      this.mainKey++;
      this.dialog = false;
    }
  },
  mounted() {
    const professionalGradients = [
      [ '#872A97', '#3F51B5' ],
      [ '#F44336', '#FFEB3B' ],
      [ '#EEAB3B', '#F44336' ],
      [ '#3F51B5', '#872A97' ],
    ];
    const randomGradient = professionalGradients[Math.floor(Math.random() * professionalGradients.length)];
    ((this.$refs['app-bar'] as any).$el as HTMLElement).style.background = `linear-gradient(to right, ${randomGradient[0]}, ${randomGradient[1]})`;
  },
}
</script>

<style scoped>
#app-bar-linear-gradient {
  /* Purple to blue */
  background: linear-gradient(to right, #872A97, #3F51B5);
}
</style>
