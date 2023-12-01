function sum(low: number, high: number): number {
  if (low > high) {
    return 0;
  }
  return high + sum(low, high - 1);
}

console.log(sum(1, 10));
