<template>
	<v-card max-width="600px">
		<v-card-title>Gib eine FEN ein</v-card-title>
		<v-card-text>
			<v-text-field v-model="fen" label="FEN" :error="error != undefined" :error-messages="error"></v-text-field>
			<div ref="board" id="chessground-fen-input"></div>
		</v-card-text>
		<v-card-actions>
			<v-spacer />
			<v-btn size="large" color="primary" @click="save" :disabled="this.error != undefined">Speichern</v-btn>
		</v-card-actions>
	</v-card>
</template>

<script lang="ts">
import { Chessground } from "chessground";
import type { Api } from "chessground/api";
import type { Key } from "chessground/types";
import { Chess } from 'chess.js';

export default {
	name: "FenDialog",
	data: () => ({
		fen: "",
		error: undefined as undefined | string,
		valid: false,
		cg: null as Api | null,
		chess: null as Chess | null,
	}),
	emits: ["done"],
	mounted() {
		const config = {
		  fen: this.fen,
		  viewOnly: true,
		  coordinates: false,
		  resizable: false,
		  addPieceZIndex: true,
		  highlight: {
			check: true,
		  },
		} as const;

		this.chess = new Chess(this.fen);
		this.cg = Chessground(this.$refs.board as HTMLElement, config);
	},
	methods: {
		save(){
			this.$emit("done", this.fen);
		}
	},
	watch: {
		fen(newFen, oldFen) {
			let result;
			try {
				let result = this.chess!.load(newFen);
			} catch (e) {
				let result = null;
			}
			if (result === null) {
				this.error = "Invalid FEN: " + newFen;
				return;
			}
			this.cg!.set({ fen: newFen });
			this.error = undefined;
		}
	}
}
</script>

<style>
#chessground-fen-input {
  width: 300px;
  height: 300px;
}
</style>
