package main

import "fmt"

func main() {
	fmt.Println(insertionSort([]int{7, 11, 15, 2, 1}))
}

func insertionSort(numbers []int) []int {
	for i := 1; i < len(numbers); i++ {
		j := i

		for j > 0 && numbers[j-1] > numbers[j] {
			numbers[j-1], numbers[j] = numbers[j], numbers[j-1]
			j--
		}
	}

	return numbers
}
