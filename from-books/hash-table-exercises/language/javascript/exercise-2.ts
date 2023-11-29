/*
 * Write a funciton that accepts an array of strings and returns the first duplicate value it finds.
 * For example, if the array is ["a", "b", "c", "d", "c", "e", "f"], the function should return "c",
 * since it's duplicated within the array. (You can assume that there's one pair of duplicates within the array.).
 * Make sure the function has an efficiency of O(n).
 */
function findDuplicate(arr: string[]): string {
  const hashTable: { [index: string]: number } = {};

  for (const element of arr) {
    hashTable[element] = (hashTable[element] || 0) + 1;
  }

  for (const property in hashTable) {
    if (hashTable[property] > 1) {
      return property;
    }
  }

  return "";
}
