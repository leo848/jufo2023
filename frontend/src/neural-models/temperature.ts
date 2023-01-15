export function temperature(inputs: number[], temperature: number): number[] {
  assert(temperature >= 0 && temperature <= 1);
  const preprocess = (temp: number) => - (2 / (Math.pow(temp, 1.8) - 2)) - 1;
  temperature = preprocess(temperature);
  assert(temperature >= 0 && temperature <= 1);

  if (temperature < 0.5) {
    return freeze(inputs, (0.5 - temperature) * 2);
  } else {
    return heat(inputs, (temperature - 0.5) * 2);
  }
}

function freeze(inputs: number[], coldness: number): number[] {
  assert(coldness >= 0 && coldness <= 1);
  const absolute_zero = [1, ...Array(inputs.length - 1).fill(0)];

  // Interpolate between inputs and absolute_zero linearly.
  return inputs.map((input, i) => {
    return input * (1 - coldness) + absolute_zero[i] * coldness;
  });
}

function heat(inputs: number[], hotness: number): number[] {
  assert(hotness >= 0 && hotness <= 1);
  const boiling_point = Array(inputs.length).fill(1 / inputs.length);

  // Interpolate between inputs and boiling_point linearly.
  return inputs.map((input, i) => {
    return input * (1 - hotness) + boiling_point[i] * hotness;
  });
}

function assert(condition: boolean) {
  if (!condition) {
    throw new Error("Assertion failed");
  }
}
