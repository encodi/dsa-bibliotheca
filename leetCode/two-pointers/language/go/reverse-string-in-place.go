package main

func main() {
	s := []byte("hello")
	reverseStrings(s)
	println(string(s))

	s = []byte("hello world")
	reverseStrings(s)
	println(string(s))
}

func reverseStrings(s []byte) {
	i := 0
	j := len(s) - 1
	for i < j {
		s[i], s[j] = s[j], s[i]
		i++
		j--
	}
}
