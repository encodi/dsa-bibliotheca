/*
 * Write a function that accepts an array of strings and returns the total number of characters across all the strings.
 * For example, if the input array is ["ab", "c", "def", "ghij"], the output should be 10 since there are 10 characters
 * in total.
 */
function numberOfChars(arr: string[]): number {
  if (arr.length === 1) return arr[0].length;
  let sum = arr[0].length;
  return sum + numberOfChars(arr.slice(1, arr.length));
}

function main() {
  console.log(numberOfChars(["ab", "c", "def", "ghij"]));
}

main();
