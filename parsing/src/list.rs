use std::{mem, path::Iter};

struct List<T>{
	head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;
pub struct Node<T>{
	pub	content : T,
	pub next: Link<T>
}

impl<T> Drop for List<T>{
	fn drop(&mut self) {
		let mut curr_link = self.head.take();
		while let Some(mut boxed_node) = curr_link {
			curr_link = boxed_node.next.take();
		}
	}
}
pub struct ListIntoIter<T>(List<T>);

impl<T> Iterator for ListIntoIter< T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

impl<T> IntoIterator for List<T> {
	type Item = T;
	type IntoIter = ListIntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		ListIntoIter(self)
	}
}

impl<T> List<T>{
	fn new() -> Self{
		List { head: None }
	}

	fn push(&mut self, content: T){
		let new_node = Box::new(
			Node {
				content,
				next : mem::replace(&mut self.head, None)
			}
		);
		self.head = Some(new_node);
	}

	fn pop(&mut self) -> Option<T>{
		match mem::replace(&mut self.head, None) {
			None => None,
			Some(node) => {
				self.head  = node.next;
				Some(node.content)
			}
		}
	}

	pub fn peek(&mut self) -> Option<&T>{
		self.head.as_ref()
			.map(|node|
				&node.content
			)
	}

	pub fn peek_mut(&mut self) -> Option<&mut T>{
		self.head.as_mut()
			.map(|node| &mut node.content)
	}
}


