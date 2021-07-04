use std::ptr;
struct Node<T> {
	elem: T,
	next: Option<Box<Node<T>>>,
}

struct Queue<T> {
	head: Option<Box<Node<T>>>,
	tail: *mut Node<T>,
}

impl<T> Queue<T> {
	fn new() -> Queue<T> {
		Queue {
			head: None,
			tail: ptr::null_mut(),
		}
	}
	fn push(&mut self, elem: T) {
		let mut new_node = Box::new(Node {
			elem,
			next: None,
		});
		let raw_pointer: *mut _ = &mut *new_node;
		if !self.tail.is_null() {
			unsafe {
				(*self.tail).next = Some(new_node);
			}
		} else {
			self.head = Some(new_node);
		}
		self.tail = raw_pointer;
	}
	
	fn pop(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			self.head = node.next;
			if self.head.is_none() {
				self.tail = ptr::null_mut();
			}
			node.elem
		})
	}
	
	fn peek(&self) -> Option<&T> {
		self.head.as_ref().map(|node| {
			&node.elem
		})
	}
}

#[cfg(test)]
mod test {
	use super::Queue;
	#[test]
	fn test_queue_opration() {
		let mut q = Queue::new();
		q.push(1); q.push(2); q.push(3);
		assert_eq!(q.pop(), Some(1));
		assert_eq!(q.pop(), Some(2));
		assert_eq!(q.pop(), Some(3));
		assert_eq!(q.pop(), None);
		q.push(4); q.push(5); q.push(6);
		assert_eq!(q.peek(), Some(&4));
		assert_eq!(q.pop(), Some(4));
		assert_eq!(q.peek(), Some(&5));
	}
}