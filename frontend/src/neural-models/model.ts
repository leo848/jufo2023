import type { LayersModel } from "@tensorflow/tfjs";
import * as tf from "@tensorflow/tfjs";
import type {
  CompleteOutput,
  FromToOutput,
  StandardPositionalInput,
  Evaluation,
} from "./types";

export interface Model<I, O> {
  name: string;
  predict(input: I): O;
}

type ModelInput<M> = M extends Model<infer I, any> ? I : never;
type ModelOutput<M> = M extends Model<any, infer O> ? O : never;

export const playModelNames = [
  "complete-model",
  "3m-unique-rust-model",
  "15m-unique-model",
  "puzzletrain-512neurons-4layers",
  "15mtrain-512neurons-4layers",
  "15mtrain-512neurons-4layers-2",
  "15mtrain-724neurons-4layers",
  "15mrevtrain-724neurons-4layers",
  "20mmatestrain-512neurons-4layers",
  "20mmatestrain-512neurons-4layers-1024batch",
  "15mtrain-512neurons-4layers-1024batch",
  "12Mopenings",
  "14Mmiddlegame",
  "15Mendgame",
] as const;

export type PlayModelName = typeof playModelNames[number];

export const evaluationModelNames = [
  "20mevaltrain-512neurons-4layers",
  "20mevaltrain-1024neurons-4layers",
  "20mevaltrain-512neurons-5layers",
  "20mevaltrain-1024neurons-5layers",
  "20mevaltrain-1024neurons-4layers-mae",
] as const;

export type EvaluationModelName = typeof evaluationModelNames[number];

// prettier-ignore
export type Models = {
  "original": Model<StandardPositionalInput, FromToOutput>;
  "vertical-model": Model<StandardPositionalInput, FromToOutput>;
  "ole-model": Model<StandardPositionalInput, FromToOutput>;
  "complete-model": Model<StandardPositionalInput, CompleteOutput>;
  "3m-unique-rust-model": Model<StandardPositionalInput, CompleteOutput>;
  "15m-unique-model": Model<StandardPositionalInput, CompleteOutput>;
  "puzzletrain-512neurons-4layers": Model<StandardPositionalInput, CompleteOutput>;
  "15mtrain-512neurons-4layers": Model<StandardPositionalInput, CompleteOutput>;
  "15mtrain-512neurons-4layers-2": Model<StandardPositionalInput, CompleteOutput>;
  "15mtrain-724neurons-4layers": Model<StandardPositionalInput, CompleteOutput>;
  "15mrevtrain-724neurons-4layers": Model<StandardPositionalInput, CompleteOutput>;
  "20mmatestrain-512neurons-4layers": Model<StandardPositionalInput, CompleteOutput>;
  "20mmatestrain-512neurons-4layers-2": Model<StandardPositionalInput, CompleteOutput>;
  "15mtrain-512neurons-4layers-1024batch": Model<StandardPositionalInput, CompleteOutput>;
  "20mevaltrain-512neurons-4layers": Model<StandardPositionalInput, Evaluation>;
  "20mevaltrain-1024neurons-4layers": Model<StandardPositionalInput, Evaluation>;
  "20mevaltrain-512neurons-5layers": Model<StandardPositionalInput, Evaluation>;
  "20mevaltrain-1024neurons-5layers": Model<StandardPositionalInput, Evaluation>;
  "20mevaltrain-1024neurons-4layers-mae": Model<StandardPositionalInput, Evaluation>;
  "12Mopenings": Model<StandardPositionalInput, CompleteOutput>;
  "14Mmiddlegame": Model<StandardPositionalInput, CompleteOutput>;
  "15Mendgame": Model<StandardPositionalInput, CompleteOutput>;
};

export const defaultModel = "20mmatestrain-512neurons-4layers-2";

async function loadTfModel(name: string): Promise<LayersModel> {
  return await tf.loadLayersModel(`models/${name}/model.json`);
}

let models = {} as { [K in keyof Models]: Models[K] };

export async function loadModel<K extends keyof Models>(
  name: K
): Promise<Models[K]> {
  if (models[name] != null) {
    return models[name];
  }
  const model = await loadTfModel(name);
  let predict = (input: ModelInput<Models[K]>) => {
    const prediction = model.predict(tf.tensor(input, [1, 833])) as tf.Tensor;
    const output = prediction.dataSync() as ModelOutput<Models[K]>;
    return output;
  };
  models[name] = { name, predict } as Models[K];
  return { name, predict } as Models[K];
}

export async function loadPlayModel(
  name: PlayModelName
): Promise<Model<StandardPositionalInput, CompleteOutput>> {
  return await loadModel(name);
}

export async function loadEvaluationModel(
  name: EvaluationModelName
): Promise<Model<StandardPositionalInput, Evaluation>> {
  return await loadModel(name);
}
