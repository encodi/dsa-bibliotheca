package main

import "fmt"

func main() {
	checkString := "the quick brown box jumps over a lazy dog"

	result := findMissingLetter(checkString)

	fmt.Println(result)
}

// Write a function that acceps a string that contains all the letters of the alphabet
// except one and returns the missing letter. For example, the string "the quick brown box
// jumps over a lazy dog" contains all the letters of the alphabet except the letter, "f".
// The function should have a time complexity of O(n).
func generateAlphabet() map[string]int {
	alphabet := make(map[string]int, 0)

	for i := 0; i < 26; i++ {
		char := string(rune(97 + i))
		alphabet[char] = i + 1
	}

	return alphabet
}

func findMissingLetter(sentence string) string {
	alphabetHashMap := generateAlphabet()
	sentenceMap := make(map[string]int, 0)

	for _, letter := range sentence {
		if _, exists := sentenceMap[string(letter)]; !exists {
			sentenceMap[string(letter)] = 1
		}
	}

	missingLetter := ""
	for letter := range alphabetHashMap {
		if _, exists := sentenceMap[letter]; !exists {
			missingLetter = letter
			break
		}
	}

	return missingLetter
}
