export function gini(values: number[]): number {
  // Return the Gini coefficient of an array of values
  
  values.sort((a, b) => a - b);
  const n = values.length;
  const cumulativeSum = values.reduce((arr, val) => {
    arr.push(val + (arr.length > 0 ? arr[arr.length - 1] : 0));
    return arr;
  }, [] as number[]);

  const integral = cumulativeSum.reduce((sum, val) => sum + val, 0);

  return (n + 1 - 2 * integral / cumulativeSum[cumulativeSum.length - 1]) / n;
}
