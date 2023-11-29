/*
 * Write a function that acceps a string that contains all the letters of the alphabet
 * except one and returns the missing letter. For example, the string "the quick brown box
 * jumps over a lazy dog" contains all the letters of the alphabet except the letter, "f".
 * The function should have a time complexity of O(n).
 */
function generateAlphabet(): { [index: string]: number } {
  const alphabetMap = {};
  for (let i = 0; i < 26; ++i) {
    let letter = String.fromCharCode(97 + i);
    alphabetMap[letter] = i + 1;
  }

  return alphabetMap;
}

function missingLetter(sentence: string): string {
  const alphabet: { [index: string]: number } = generateAlphabet();
  const sentenceMap: { [index: string]: number } = {};

  for (const letter of sentence) {
    if (!sentenceMap[letter]) {
      sentenceMap[letter] = 1;
    }
  }

  let missing = "";
  for (const letter in alphabet) {
    if (!sentenceMap[letter]) {
      missing = letter;
      break;
    }
  }

  return missing;
}

console.log(missingLetter("the quick brown box jumps over a lazy dog"));
