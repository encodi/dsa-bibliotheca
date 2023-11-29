/*
 * Write a function that returns the first non-duplicated character in a string.
 * For example, the string "minimum" has two characters that only exist once, the "n" and "u",
 * so your function should return the "n", since it occurs first. The function should have an efficiency of O(n).
 */
function firstNonDuplicate(sentence: string): string {
  const hashMap: { [index: string]: number } = {};

  for (const letter of sentence) {
    hashMap[letter] = (hashMap[letter] || 0) + 1;
  }

  for (const letter in hashMap) {
    if (hashMap[letter] === 1) {
      return letter;
    }
  }

  return "";
}
