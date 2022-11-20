<template>
  <div>
    <h1>My App</h1>
    <p>{{ message }}</p>
  </div>
</template>

<script lang="ts">
export default {
  data: () => ({
    message: 'Hello World!',
  }),
}
</script>
