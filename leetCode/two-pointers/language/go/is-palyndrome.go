package main

import "fmt"

func main() {
	fmt.Println(isPalindrome("abba"))
	fmt.Println(isPalindrome("abbaa"))
}

func isPalindrome(s string) bool {
	if len(s) == 0 {
		return true
	}

	left := 0
	right := len(s) - 1

	for left < right {
		if s[left] != s[right] {
			return false
		}

		left++
		right--
	}

	return true
}
