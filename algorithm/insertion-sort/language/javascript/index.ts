function insertionSort(arr) {
  for (let i = 1; i < arr.length; i++) {
    const currentVal = arr[i];
    let j = i - 1;

    while (j >= 0 && currentVal < arr[j]) {
      arr[j + 1] = arr[j];
      j--;
    }

    arr[j + 1] = currentVal;
  }

  return arr;
}

console.log(insertionSort([5, 4, 3, 2, 1]));
