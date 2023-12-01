function printNumbers(data: any): void {
  if (Array.isArray(data)) {
    data.forEach((item) => printNumbers(item));
  } else {
    console.log(data);
  }
}

function main(): void {
  const mixed = [1, 2, 3, [4, 5, 6], 7, 8, [9, [10, 11]]];

  printNumbers(mixed);
}

main();
