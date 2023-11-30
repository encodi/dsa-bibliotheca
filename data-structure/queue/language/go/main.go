package main

import "fmt"

type Queue[T any] struct {
	items []T
}

func (q *Queue[T]) Enqueue(item T) {
	q.items = append(q.items, item)
}

func (q *Queue[T]) Dequeue() T {
	var zero T
	if len(q.items) == 0 {
		return zero
	}
	item := q.items[0]
	q.items = q.items[1:]
	return item
}

func (q *Queue[T]) IsEmpty() bool {
	return len(q.items) == 0
}

func (q *Queue[T]) Len() int {
	return len(q.items)
}

func (q *Queue[T]) Peek() T {
	var zero T
	if len(q.items) == 0 {
		return zero
	}
	return q.items[0]
}

func (q *Queue[T]) Clear() {
	q.items = []T{}
}

func (q *Queue[T]) ToString() string {
	return fmt.Sprintf("Queue: %v", q.items)
}

func main() {
	queue := Queue[int]{}
	queue.Enqueue(1)
	queue.Enqueue(2)
	queue.Enqueue(3)
	fmt.Println(queue.ToString())
	fmt.Println(queue.Dequeue())
	fmt.Println(queue.ToString())
	fmt.Println(queue.Peek())
	fmt.Println(queue.Len())
	fmt.Println(queue.IsEmpty())
	queue.Clear()
}
