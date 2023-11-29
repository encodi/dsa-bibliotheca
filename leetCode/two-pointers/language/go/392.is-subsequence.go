package main

import "fmt"

func main() {
	s := "abc"
	t := "ahbgdc"
	fmt.Println(isSubsequence(s, t))
}

func isSubsequence(s string, t string) bool {
	if len(s) == 0 {
		return true
	}
	i := 0
	for j := 0; j < len(t); j++ {
		if t[j] == s[i] {
			i++
			if i == len(s) {
				return true
			}
		}
	}
	return false
}
