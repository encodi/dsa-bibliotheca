function isPalyndrome(word: string): boolean {
  let left = 0;
  let right = word.length - 1;

  while (left < right) {
    if (word[left] !== word[right]) {
      return false;
    }

    left++;
    right--;
  }

  return true;
}

const test1 = isPalyndrome("abba");
console.log(test1);

const test2 = isPalyndrome("abbaa");
console.log(test2);
