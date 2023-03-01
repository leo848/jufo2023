<template>
  <v-app>
    <v-app-bar ref="app-bar" app floating>
      <v-app-bar-title>
        Wie kann Künstliche Intelligenz Schach lernen?
      </v-app-bar-title>
      <v-spacer></v-spacer>
      <v-btn icon href="https://raw.githubusercontent.com/leo848/jufo2023/master/langfassung.pdf">
        <v-icon>mdi-file-document-multiple</v-icon>
        <v-tooltip activator="parent" location="bottom">
          <span>Langfassung</span>
        </v-tooltip>
      </v-btn>
      <v-btn icon href="https://github.com/leo848/jufo2023">
        <v-icon>mdi-code-braces-box</v-icon>
        <v-tooltip activator="parent" location="bottom">
          <span>Quellcode</span>
        </v-tooltip>
      </v-btn>
      <v-btn icon class="mx-4">
        <v-icon size="large">mdi-cog</v-icon>
        <v-dialog v-model="dialog" activator="parent" max-width="800px">
          <SettingsDialog @cancel="cancel" @save="save" />
        </v-dialog>
      </v-btn>
    </v-app-bar>
    <v-main class="mt-4" :key="mainKey">
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import SettingsDialog from "@/components/SettingsDialog.vue";

export default {
  name: "App",
  components: { SettingsDialog },
  data: () => ({
    dialog: false,
    mainKey: 0,
  }),
  methods: {
    cancel() {
      this.dialog = false;
    },
    save() {
      this.mainKey++;
      this.dialog = false;
    },
  },
  mounted() {
    const professionalGradients = [
      ["#872A97", "#3F51B5"],
      ["#EEAB3B", "#F44336"],
      ["#3F51B5", "#872A97"],
    ] as const;
    const randomGradient =
      professionalGradients[
        Math.floor(Math.random() * professionalGradients.length)
      ];
    (
      (this.$refs["app-bar"] as any).$el as HTMLElement
    ).style.background = `linear-gradient(to right, ${randomGradient[0]}, ${randomGradient[1]})`;
  },
};
</script>

<style scoped>
#app-bar-linear-gradient {
  /* Purple to blue */
  background: linear-gradient(to right, #872a97, #3f51b5);
}
</style>

<style lang="sass">
@import '../node_modules/@fontsource/roboto/index.css'
</style>
