package main

import "fmt"

func main() {
	str := "minimum"

	result := firstNonDuplicate(str)

	fmt.Println(result)
}

// Write a function that returns the first non-duplicated character in a string.
// For example, the string "minimum" has two characters that only exist once, the "n" and "u",
// so your function should return the "n", since it occurs first. The function should have an efficiency of O(n).
func firstNonDuplicate(sentence string) string {
	hashMap := map[string]int{}

	for _, letter := range sentence {
		if _, exists := hashMap[string(letter)]; !exists {
			hashMap[string(letter)] = 0
		}
		hashMap[string(letter)]++
	}

	for letter := range hashMap {
		if hashMap[letter] == 1 {
			return letter
		}
	}

	return ""
}
