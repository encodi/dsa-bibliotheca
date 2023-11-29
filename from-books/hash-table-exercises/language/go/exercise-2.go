package main

import "fmt"

// Write a funciton that accepts an array of strings and returns the first duplicate value it finds.
// For example, if the array is ["a", "b", "c", "d", "c", "e", "f"], the function should return "c",
// since it's duplicated within the array. (You can assume that there's one pair of duplicates within the array.).
// Make sure the function has an efficiency of O(n).
func main() {
	arr := []string{"a", "b", "c", "d", "c", "e", "f"}

	result := firstDuplicate(arr)

	fmt.Printf(result)
}

func firstDuplicate(arr []string) string {
	hashMap := make(map[string]int, 0)

	for _, letter := range arr {
		if _, exists := hashMap[letter]; !exists {
			hashMap[letter] = 0
		}
		hashMap[letter]++
	}

	for letter, value := range hashMap {
		if value > 1 {
			return letter
		}
	}

	return ""
}
