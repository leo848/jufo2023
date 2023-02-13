<template>
  <span class="ml-2">
    <v-menu max-width="350px">
      <!-- background color black -->
      <v-card>
        <v-card-title class="text-h5">{{ title }}</v-card-title>
        <v-card-text class="text-body-1">{{ text }}</v-card-text>
      </v-card>
      <template v-slot:activator="{ props }">
        <v-icon @click.stop v-bind="props" v-if="show">mdi-information</v-icon>
      </template>
    </v-menu>
  </span>
</template>

<script lang="ts">
import type { MoveWithAct } from "@/neural-models/types";
import { loadSetting } from '@/settings/settings';

type LookupResult = {
  title: string;
  template: string;
  templateFunction?: (templates: any) => string;
  prepare?: (templates: any) => any;
};

const lookup = {
  "move-suggestions": {
    title: "Zugvorschläge",
    template:
      "Die Zugvorschläge zeigen alle Züge, die das Modell in der aktuellen Stellung ausgibt. Sie sind nach der neuronalen Aktivierung sortiert und, falls ausgewählt, nach ihrer Legalität im Schach gefiltert.",
  } as LookupResult,
  "legal-outputs": {
    title: "Legale Ausgaben",
    template:
      "Das Modell hat immer 4096 Ausgaben, die jeweils einem Zug entsprechen. Die Softmax-Funktion garantiert, dass sie zusammen immer 1 bzw. 100% ergeben. Der (hoffentlich) grüne Kreis zeigt nun an, wie viele der Ausgaben legal und wieviele illegal sind – jetzt gerade sind es {percentage}%.",
  } as LookupResult,
  "move-display": {
    title: "Zuganzeige",
    templateFunction: (template: { move: MoveWithAct }) => {
      const { move } = template;
      if (!move.inner) {
        return `Dieser Zug ist nicht legal - in dieser Stellung kann nicht von ${
          move.from
        } nach ${
          move.to
        } gezogen werden. Hierbei beträgt die neuronale Aktivierung dieses (illegalen) Zuges ${(
          move.act * 100
        ).toFixed(3)}%.`;
      }
      let piece = {
        p: "Bauer",
        n: "Springer",
        b: "Läufer",
        r: "Turm",
        q: "Dame",
        k: "König",
      }[move.inner.piece];
      return `Der hier vorgeschlagene Zug ist ${piece} von ${move.from} nach ${
        move.to
      }. Mit einer neuronalen Aktivierung dieses Zuges von ${(move.act * 100).toFixed(
        3
      )}% ist er der ${move.index}. Zug in der Liste.`;
    },
  } as LookupResult,
  "game-over": {
    title: "Spielende",
    template: "Eine Schachpartie kann durch viele verschiedene Gründe enden, aber es gibt immer drei Ausgänge: Schwarz gewinnt, Weiß gewinnt oder Remis (Unentschieden). Ein Gewinn kann durch ein Schachmatt oder durch vorzeitige Aufgabe eines Spielers entstehen, ein Remis durch Patt, ungenügendes Material oder dreimal die gleiche Stellung."
  }
} as const;

const keyValidator = (name: string): name is keyof typeof lookup =>
  name in lookup;

export default {
  name: "InfoBox",
  props: {
    name: {
      type: String,
      required: true,
      validator: keyValidator,
    },
    templates: {
      type: Object,
      required: false,
      default: () => ({}),
    },
    directText: {
      type: String,
      required: false,
      default: null,
    },
  },
  data: () => ({
    show: loadSetting("show").infoboxes,
  }),
  computed: {
    text(): string {
      if (this.directText) {
        return this.directText;
      }
      if (!keyValidator(this.name)) {
        throw new Error(`Invalid key: ${this.name}`);
      }
      if (!this.templates) {
        throw new Error("No templates");
      }
      const result = lookup[this.name];
      let templates = this.templates;

      if (result.prepare) {
        templates = result.prepare(templates);
      }

      if (result.templateFunction) {
        return result.templateFunction(templates);
      }

      function getTemplate(key: string): string {
        const template = templates[key];
        if (!template) {
          console.log(templates);
          throw new Error(`No template for ${key}`);
        }
        return template;
      }
      return result.template.replace(/{(\w+)}/g, (_, key) =>
        getTemplate.call(this, key)
      );
    },
    title(): string {
      if (!keyValidator(this.name)) {
        throw new Error(`Invalid key: ${this.name}`);
      }
      return lookup[this.name].title;
    },
  },
};
</script>
