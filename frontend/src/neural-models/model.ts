import type {LayersModel} from '@tensorflow/tfjs';
import * as tf from '@tensorflow/tfjs';
import type {CompleteOutput, FromToOutput, StandardPositionalInput, Evaluation} from './types';

export interface Model<I, O> {
  name: string;
  predict(input: I): O;
}

type ModelInput<M> = M extends Model<infer I, any> ? I : never;
type ModelOutput<M> = M extends Model<any, infer O> ? O : never;

export type Models = {
  "original": Model<StandardPositionalInput, FromToOutput>
  "vertical-model": Model<StandardPositionalInput, FromToOutput>
  "ole-model": Model<StandardPositionalInput, FromToOutput>
  "complete-model": Model<StandardPositionalInput, CompleteOutput>
  "3m-unique-rust-model": Model<StandardPositionalInput, CompleteOutput>
  "15m-unique-model": Model<StandardPositionalInput, CompleteOutput>
  "puzzletrain-512neurons-4layers": Model<StandardPositionalInput, CompleteOutput>
  "15mtrain-512neurons-4layers": Model<StandardPositionalInput, CompleteOutput>
  "15mtrain-512neurons-4layers-2": Model<StandardPositionalInput, CompleteOutput>
  "15mtrain-724neurons-4layers": Model<StandardPositionalInput, CompleteOutput>
  "15mrevtrain-724neurons-4layers": Model<StandardPositionalInput, CompleteOutput>
  "20mmatestrain-512neurons-4layers": Model<StandardPositionalInput, CompleteOutput>
  "20mmatestrain-512neurons-4layers-2": Model<StandardPositionalInput, CompleteOutput>
  "15mtrain-512neurons-4layers-1024batch": Model<StandardPositionalInput, CompleteOutput>
  "20mevaltrain-512neurons-4layers": Model<StandardPositionalInput, Evaluation>
}

export type EvaluationModel = "20mevaltrain-512neurons-4layers";

export const defaultModel = "20mmatestrain-512neurons-4layers-2";

async function loadTfModel(name: string): Promise<LayersModel> {
  return await tf.loadLayersModel(`models/${name}/model.json`);
}

let models = {} as { [K in keyof Models]: Models[K] };

export async function loadModel<K extends keyof Models>(name: K): Promise<Models[K]> {
  if (models[name] != null) {
    return models[name];
  }
  const model = await loadTfModel(name);
  let predict = (input: ModelInput<Models[K]>) => {
    const prediction = model.predict(tf.tensor(input, [1, 833])) as tf.Tensor;
    const output = prediction.dataSync() as ModelOutput<Models[K]>;
    return output;
  };
  models[name] = {name, predict} as Models[K];
  return { name, predict } as Models[K];
}
