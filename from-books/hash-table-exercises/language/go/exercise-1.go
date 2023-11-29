package main

import "fmt"

func main() {
	arr1 := []int{1, 2, 3, 4}
	arr2 := []int{3, 4, 5, 6}

	result := intersection(arr1, arr2)

	fmt.Println(result)
}

// Write a function that returns the intersection of two arrays. The intersection is a third array
// that contains all values contained within the first two arrays. For example, the intersection
// of [1, 2, 3] and [2, 3, 4] is [2, 3]. Your function should have a complexity of O(n). Do not use
// any built-in functions in the language used.
func intersection(arr1 []int, arr2 []int) []int {
	hashMap := make(map[int]int)

	for _, value := range arr1 {
		if _, exists := hashMap[value]; !exists {
			hashMap[value] = 0
		}
		hashMap[value]++
	}

	ans := make([]int, 0)
	for _, value := range arr2 {
		if hashMap[value] > 0 {
			ans = append(ans, value)
		}
	}

	return ans
}
