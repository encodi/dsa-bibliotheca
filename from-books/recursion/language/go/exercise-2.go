package main

import (
	"fmt"
)

func main() {
	mixed := []interface{}{
		1, 2, 3,
		[]int{4, 5, 6},
		7, 8,
		[]interface{}{
			9,
			[]int{10, 11},
		},
	}

	printNumbers(mixed)
}

// Create a function that prints all the numbers in a mixed array with multiple layers.

func printNumbers(data interface{}) {
	switch v := data.(type) {
	case int:
		fmt.Println(v)
	case []interface{}:
		for _, item := range v {
			printNumbers(item)
		}
	case []int:
		for _, item := range v {
			printNumbers(item)
		}
	}
}
