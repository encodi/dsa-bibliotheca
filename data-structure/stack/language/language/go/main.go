package main

import "fmt"

type Stack[T any] struct {
	items []T
}

func (s *Stack[T]) Push(item T) {
	s.items = append(s.items, item)
}

func (s *Stack[T]) Pop() T {
	var zero T
	if len(s.items) == 0 {
		return zero
	}
	item := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return item
}

func (s *Stack[T]) IsEmpty() bool {
	return len(s.items) == 0
}

func (s *Stack[T]) Len() int {
	return len(s.items)
}

func (s *Stack[T]) Peek() T {
	var zero T
	if len(s.items) == 0 {
		return zero
	}
	return s.items[len(s.items)-1]
}

func (s *Stack[T]) Clear() {
	s.items = []T{}
}

func (s *Stack[T]) ToString() string {
	return fmt.Sprintf("Stack: %v", s.items)
}

func main() {
	stack := Stack[int]{}
	stack.Push(1)
	stack.Push(2)
	stack.Push(3)
	fmt.Println(stack.ToString())
	fmt.Println(stack.Pop())
	fmt.Println(stack.ToString())
	fmt.Println(stack.Peek())
	fmt.Println(stack.Len())
	fmt.Println(stack.IsEmpty())
	stack.Clear()
}
