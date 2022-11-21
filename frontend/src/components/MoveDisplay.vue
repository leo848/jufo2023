<template>
  <div>
    <div v-for="move in moves" :key="move.from + move.to">
      <span>{{ move.from }} -&gt; {{ move.to }}</span>
      <span>{{ move.act }}</span>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'MoveDisplay',
  props: {
    moves: {
      type: Array,
      required: true
    }
  }
}
</script>

<style>

</style>
