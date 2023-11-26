package main

import "fmt"

func main() {
	fmt.Println(selectionSort([]int{7, 11, 15, 2, 1}))
}

func selectionSort(numbers []int) []int {
	for i := 0; i < len(numbers); i++ {
		min := i

		for j := i + 1; j < len(numbers); j++ {
			if numbers[j] < numbers[min] {
				min = j
			}
		}

		numbers[i], numbers[min] = numbers[min], numbers[i]
	}

	return numbers
}
