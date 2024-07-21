use core::{fmt, str};
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Node {
	pub name: &'static str,
	pub left: Vec<Option<Rc<RefCell<Node>>>>,
	pub right: Vec<Option<Rc<RefCell<Node>>>>,
	pub is_directory: bool
}

impl Node {

	pub fn new(name: &'static str) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(Node {
		    name,
		    left: Vec::new(),
		    right: Vec::new(),
		    is_directory: false,
		}))
	}

	pub fn set_left(mut self, name: &'static str) {
		let new_node:Rc<RefCell<Node>> = Node::new(name);
		self.borrow_mut().left.push(Some(new_node));
	}

	pub fn set_right(mut self, name: &'static str) {
		let new_node:Rc<RefCell<Node>> = Node::new(name);
		self.borrow_mut().right.push(Some(new_node));
	}
}


impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	    f.debug_struct("DirectoryNode")
		.field("val", &self.name)
		.field("left", &self.left)
		.field("right", &self.right)
		.field("node_type", &self.is_directory)
		// Add more fields as needed
		.finish()
	}
}
