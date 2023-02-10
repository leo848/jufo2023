<template>
  <v-tooltip location="bottom">
    <p class="text-h4">{{ display(evaluation) }}</p>
    <template v-slot:activator="{ props }">
      <v-progress-linear
        v-model="displayEvaluation"
        :max="1.0"
        :min="0.0"
        color="white"
        background-color="black"
        height="20"
        :indeterminate="model === null"
        class="rounded-xl mt-4"
        v-bind="props"
        >
      </v-progress-linear>
    </template>
  </v-tooltip>
</template>

<script lang="ts">
import { loadEvaluationModel, type Model } from '@/neural-models/model';
import type { StandardPositionalInput, Evaluation } from '@/neural-models/types';
import { fenToStandardPositionalInput, evaluationOutputToEvaluation } from '@/neural-models/chess_conversions';
import { loadSetting } from '@/settings/settings';

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
  created() {
    loadEvaluationModel(loadSetting("evaluationModelName"))
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
  computed: {
    displayEvaluation(): number {
      if (this.evaluation >= 1) return 1;
      if (this.evaluation <= 0) return 0;
      return this.sigmoid(this.logit(this.evaluation), 3 / 5)
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
    },
    logit(x: number): number {
      return Math.log(x / (1 - x));
    },
    sigmoid(x: number, slope: number = 1): number {
      return 1 / (1 + Math.exp(-slope * x));
    },
    display(evaluation: number): string {
      const logit = this.logit(evaluation);
      const numberFormat = new Intl.NumberFormat(undefined, {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
        signDisplay: "always",
      });
      if (isNaN(logit)) {
        if (evaluation <= 0) return numberFormat.format(-Infinity);
        if (evaluation >= 1) return numberFormat.format(Infinity);
      }
      return numberFormat.format(logit);
    }
  }
}
</script>
