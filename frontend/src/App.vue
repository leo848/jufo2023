<template>
  <v-app>
    <v-app-bar app floating id="app-bar-linear-gradient">
      <v-app-bar-title>Chess using neural networks</v-app-bar-title>
      <v-spacer></v-spacer>
      <v-btn icon>
        <v-icon>mdi-cog</v-icon>
      </v-btn>
    </v-app-bar>
    <v-main class="mt-4">
      <router-view/>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'App',
})
</script>

<style scoped>
#app-bar-linear-gradient {
  /* Purple to blue */
  background: linear-gradient(to right, #872A97, #3F51B5);
}
</style>
