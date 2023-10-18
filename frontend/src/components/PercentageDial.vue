<template>
<v-tooltip right :open-on-click="true">
  <p class="text-h4">
    {{ displayValue }}
  </p>
  <template v-slot:activator="{ props }">
    <v-progress-circular
      :model-value="value * 100"
      color="green"
      bg-color="red"
      width="8"
      size="large"
      class="ml-4"
      v-bind="props"
    />
  </template>
</v-tooltip>
</template>

<script lang="ts">
export default {
  name: "PercentageDial",
  props: {
    value: {
      type: Number,
      required: true,
    }
  },
  computed: {
    displayValue() {
      const numberFormat = new Intl.NumberFormat(undefined, {
        style: "percent",
        maximumFractionDigits: 2,
        minimumFractionDigits: 2,
      });
      return numberFormat.format(this.value);
    }
  }
}
</script>
