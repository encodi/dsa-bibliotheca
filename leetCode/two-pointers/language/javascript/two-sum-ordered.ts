function twoSum(numbers: number[], target: number): number[] {
  let left = 0;
  let right = numbers.length - 1;

  while (left < right) {
    const sum = numbers[left] + numbers[right];

    if (sum === target) {
      return [left, right];
    }

    if (sum < target) {
      left++;
    } else {
      right--;
    }
  }

  return [];
}

const test3 = twoSum([2, 7, 11, 15], 9);
console.log(test3);
const test4 = twoSum([2, 3, 4], 6);
console.log(test4);
