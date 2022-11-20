<template>
  <div>
    <h1>My App</h1>
    <p>{{ message }}</p>
  </div>
</template>

<script lang="ts">
import { loadFromToOutputModel } from '@/neural-models/model'
import { fromToOutputToMoves } from '@/neural-models/chess_conversions'
export default {
  async setup() {
    const model = await loadFromToOutputModel();
  },
  data: () => ({
    message: 'Hello World!',
  }),
}
</script>
