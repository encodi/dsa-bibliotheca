package main

import "fmt"

// Write a function that accepts an array of strings and returns the total number of characters across all the strings.
// For example, if the input array is ["ab", "c", "def", "ghij"], the output should be 10 since there are 10 characters
// in total.

func numberOfChars(arr []string) int {
	if len(arr) == 1 {
		return len(arr[0])
	}

	sum := len(arr[0])

	return sum + numberOfChars(arr[1:])
}

func main() {
	fmt.Println(([]string{"ab", "c", "def", "ghij"}))
}
