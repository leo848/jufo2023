export function temperature(inputs: number[], temperature: number): number[] {
  assert(temperature >= 0 && temperature <= 2);
  if (temperature < 1) {
    return freeze(inputs, 1 - temperature);
  } else {
    return heat(inputs, temperature - 1);
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
