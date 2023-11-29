/*
 * Write a function that returns the intersection of two arrays. The intersection is a third array
 * that contains all values contained within the first two arrays. For example, the intersection
 * of [1, 2, 3] and [2, 3, 4] is [2, 3]. Your function should have a complexity of O(n). Do not use
 * any built-in functions in the language used.
 */

function intersection(arr1: number[], arr2: number[]): number[] {
  const pseudoHashTable: { [index: number]: boolean } = {};

  for (let i = 0; i < arr1.length; ++i) {
    pseudoHashTable[arr1[i]] = true;
  }

  const ans: number[] = [];
  for (let j = 0; j < arr2.length; ++j) {
    if (pseudoHashTable[arr2[j]]) {
      ans.push(arr2[j]);
    }
  }

  return ans;
}

console.log(intersection([1, 2, 3], [2, 3, 4]));
