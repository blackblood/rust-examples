struct Queue<T> {
	head: Option<Box<Node<T>>>,
	tail: Option<Box<Node<T>>>,
	max_size: u32,
	current_size: u32
}

struct Node<T> {
	value: Option<T>,
	next: Option<Box<Node<T>>>,
	prev: Option<Box<Node<T>>>
}

impl<T> Node<T> {
	fn new(val: Option<T>) -> Node<T> {
		Node { value: val, next: None, prev: None }
	}

	fn set_next(mut self, n: Option<Box<Node<T>>>) {
		self.next = n;
	}
}

impl<T> Queue<T> {
	fn new() -> Queue<T> {
		Queue { head: None, tail: None, max_size: 5, current_size: 0 }
	}
	
	fn set_size(mut self, s: u32) {
		self.max_size = s;
	}

	fn enqueue(&mut self, el: T) {
		let mut n = Node::new(Some(el));
		n.next = self.head;
		self.head = Some(Box::new(n));
		self.current_size += 1;
		if let Some(mut t) = &self.tail {
			if self.current_size > self.max_size {
				t.next = None;
				self.tail = t.prev;
				self.current_size -= 1;
			}
		}
	}

	fn deque(mut self) -> Option<Box<Node<T>>> {
		if let Some(h) = self.head {
			if let Some(n) = h.next {
				self.head = Some(n);
				self.current_size -= 1;
				return self.head;
			}
		}
		return None;
	}
}

fn main() {
	let mut q = Queue::new();
	q.enqueue(100);
	q.enqueue(200);
	// q.enqueue(300);
}