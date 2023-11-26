function binarySearch(arr: number[], target: number): number {
  let left = 0;
  let right = arr.length - 1;
  let mid = Math.floor((right + left) / 2);

  while (left <= right) {
    if (arr[mid] === target) return mid;
    if (arr[mid] < target) left = mid + 1;
    if (arr[mid] > target) right = mid - 1;
    mid = Math.floor((right + left) / 2);
  }

  return -1;
}
