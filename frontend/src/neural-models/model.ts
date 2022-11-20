import type {LayersModel} from '@tensorflow/tfjs';
import * as tf from '@tensorflow/tfjs';
import type {FromToOutput, StandardPositionalInput} from './types';

export interface Model<I, O> {
  name: string;
  predict(input: I): O;
}

export async function loadModel(name: string): Promise<LayersModel> {
  return await tf.loadLayersModel(`models/${name}/model.json`);
}

export async function loadFromToOutputModel(name="original"): Promise<Model<StandardPositionalInput, FromToOutput>> {
  const model = await loadModel(name);
  return {
    name,
    predict: (input: StandardPositionalInput) => {
      const prediction = model.predict(tf.tensor(input, [1, 833])) as tf.Tensor;
      const output = prediction.dataSync() as FromToOutput;
      return output;
    },
  };
}
