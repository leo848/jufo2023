<template>
  <v-progress-linear
    v-if="model"
    v-model="evaluation"
    :max="1.0"
    :min="0.0"
    color="white"
    background-color="black"
    height="20"
    rounded
  />
</template>

<script lang="ts">
import { loadModel, type Model } from '@/neural-models/model';
import type { StandardPositionalInput, Evaluation } from '@/neural-models/types';
import { fenToStandardPositionalInput, evaluationOutputToEvaluation } from '@/neural-models/chess_conversions';

export default {
  name: 'Evaluation',
  data: () => ({
    model: null as Model<StandardPositionalInput, Evaluation> | null,
    evaluation: 0.5,
  }),
  props: {
    fen: {
      type: String,
      required: true,
    },
  },
  async created() {
    loadModel("20mevaltrain-512neurons-4layers")
      .then(model => {
        this.model = model;
        this.updateEvaluation();
      })
  },
  watch: {
    fen() {
      this.updateEvaluation();
    }
  },
  methods: {
    updateEvaluation() {
      if (!this.model) {
        return;
      }
      const positionalInput = fenToStandardPositionalInput(this.fen);
      const evaluation = this.model.predict(positionalInput);
      this.evaluation = evaluationOutputToEvaluation(evaluation);
    }
  }
}
</script>
