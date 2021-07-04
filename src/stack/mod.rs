#[derive(Debug)]
pub struct Node<T> {
	elem: T,
	next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
	head: Option<Box<Node<T>>>,
}

pub struct IterImpl<'a, T> {
	next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for IterImpl<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.map(|node| {
			self.next = node.next.as_deref();
			&node.elem
		})
	}
}

pub struct IterMutImpl<'a, T> {
	next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMutImpl<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.next.as_deref_mut();
			&mut node.elem
		})
	}
}

impl<T> Stack<T> {
	pub fn new() -> Stack<T> {
		Stack { head: None}
	}
	
	pub fn iter_mut(&mut self) -> IterMutImpl<T> {
		return IterMutImpl {
			next: self.head.as_deref_mut(),
		}
	}
	
	pub fn iter(&self) -> IterImpl<T> {
		return IterImpl {
			next: self.head.as_deref(),
		}
	}
	
	pub fn push(&mut self, elem: T) {
		let new_node = Some(Box::new(Node {
			elem,
			next: self.head.take(),
		}));
		self.head = new_node;
	}
	
	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			self.head = node.next;
			node.elem
		})
	}
	
	pub fn peek(&mut self) -> Option<&T> {
		self.head.as_ref().map(|node| {
			&node.elem
		})
	}
}

impl<T> Drop for Stack<T> {
	fn drop(&mut self) {
		println!("Starting Destroy Stack.");
		let mut node = self.head.take();
		while let Some(mut tmp_node) = node {
			node = tmp_node.next.take();
		}
	}
}

#[cfg(test)]
mod test {
	use super::Stack;
	#[test]
	fn test_basic_operation() {
		let mut stk = Stack::new();
		stk.push(1);
		stk.push(2);
		stk.push(3);
		
		assert_eq!(stk.pop(), Some(3));
		assert_eq!(stk.pop(), Some(2));
// 		assert_eq!(stk.pop(), Some(2));
		
		stk.push(5);
		stk.push(4);
		assert_eq!(stk.peek(), Some(&4));
		
		let mut strstk = Stack::new();
		strstk.push("1");
		strstk.push("2");
		strstk.push("3");
		
		assert_eq!(strstk.pop(), Some("3"));
		assert_eq!(strstk.pop(), Some("2"));

	}
	#[test]
	fn test_iterator() {
		let mut stk = Stack::new();
		stk.push(1); stk.push(2); stk.push(3);
		let mut it = stk.iter();
// 		it.next().insert(&12);
		assert_eq!(it.next(), Some(&3));
		assert_eq!(it.next(), Some(&2));
		assert_eq!(it.next(), Some(&1));
		assert_eq!(it.next(), None);
		// 遍历stack.
		it = stk.iter();
		loop {
			let mut elem = it.next();
			if elem == None {
				break;
			}
			println!("{:?}", elem.take());
		}
	}
	
	#[test]
	fn test_mut_iterator() {
		let mut stk = Stack::new();
		stk.push(1); stk.push(2); stk.push(3);
		let mut it = stk.iter_mut();
		println!("{}", "Start Testing Mut Iterator.");
		assert_eq!(it.next(), Some(&mut 3));
		assert_eq!(it.next(), Some(&mut 2));
		assert_eq!(it.next(), Some(&mut 1));
		assert_eq!(it.next(), None);
	}
	
	#[test]
	fn test_it_with_obj() {
		let mut stk = Stack::new();
		stk.push(User{name: String::from("Linhd"), age: 22});
		stk.push(User{name: String::from("Scathon"), age: 23});
		let mut it = stk.iter();
		let user_op = it.next();
		println!("{:?}", user_op);
		// this is wrong, rust doesn't allow us to modify the name of user.
		// because user is a & reference not a mut reference.
// 		one_user.map(|user| { user.name = String::from("LinHuadong")});
		let mut it = stk.iter_mut();
		let mut user_op : Option<&mut User> = it.next();
		user_op.map(|user| {user.name = String::from("LinHuadong")});
		it = stk.iter_mut(); // reset iterator.
		user_op = it.next();
		println!("{:?}", user_op);
		println!("{:?}", user_op);
	}
	
	#[test]
	fn test_pop() {
		println!("Start Testing Pop Function.");
		let mut stk = Stack::new();
		stk.push(1); stk.push(2);
		stk.pop();
		println!("{:?}", stk.peek());
	}
	
	#[derive(Debug)]
	struct User {
		name: String,
		age: i32,
	}
	
}
