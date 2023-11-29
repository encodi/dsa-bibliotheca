function reverseString(str: string): string {
  const arr = str.split("");
  let i = 0;
  let j = arr.length - 1;
  while (i < j) {
    [arr[i], arr[j]] = [arr[j], arr[i]];
    i++;
    j--;
  }
  return arr.join("");
}

console.log(reverseString("hello"));
