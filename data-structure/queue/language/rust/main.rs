use std::collections::VedDeque;

struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
