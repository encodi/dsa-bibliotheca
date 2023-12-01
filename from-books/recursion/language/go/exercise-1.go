package main

import "fmt"

// Write a function that recieves two  numbers called low and high, the function returns the sum of all the numbers
// from low to high.

func main() {
	fmt.Println(sum(1, 10))
}

func sum(low int, high int) int {
	if low > high {
		return 0
	}
	return high + sum(low, high-1)
}
