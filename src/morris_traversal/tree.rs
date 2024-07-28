use std::{borrow::Borrow, cell::RefCell, path::is_separator, rc::Rc};

use super::node::{Directory, Node, OnlyDirectory};


#[derive(Debug)]
pub struct Tree {
	root: Option<Rc<RefCell<Directory>>>
}

pub trait BinaryTree {
	fn new(path_name:String, node_parent:Option<String>) -> Self;
	fn get_root(&self) -> Option<Rc<RefCell<Directory>>>;
	fn insert(&mut self, path_name:String, node_parent:Option<String>);
}

impl BinaryTree for Tree {
	fn new(path_name:String, node_parent:Option<String>) -> Self{
		let new_root = Directory::new(path_name, node_parent);
		Tree {
			root: Some(new_root)
		}
	}

	fn get_root(&self) -> Option<Rc<RefCell<Directory>>>{
		self.root.clone()
	}

	fn insert(&mut self, path_name:String, node_parent:Option<String>){
		if let Some(current) = self.root.borrow() {
			current.borrow_mut().add_new_child(path_name, node_parent)
		}
	}
}

