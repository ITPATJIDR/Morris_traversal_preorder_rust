use core::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub struct DirectoryNode {
	pub val: &'static str,
	pub left: Option<Rc<RefCell<FileNode>>>,
	pub right: Option<Rc<RefCell<DirectoryNode>>>,
}

pub struct FileNode {
	pub val: &'static str,
	pub left: Option<Rc<RefCell<FileNode>>>,
}

impl DirectoryNode {
	pub fn create_tree() -> Self {
		let root_node = DirectoryNode{
			val: "/home",
			left: Some(Rc::new(RefCell::new(FileNode { val: "/test.js", left: None,}))),
			right: Some(Rc::new(RefCell::new(DirectoryNode { val: "/src", left: None, right: None})))
		};

		println!("val: {:?}, left: {:?}, right: {:?}", root_node.val, root_node.right, root_node.left);
		root_node
	}
}


impl fmt::Debug for DirectoryNode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	    f.debug_struct("DirectoryNode")
		// .field("field_name", &self.field_name)
		// Add more fields as needed
		.finish()
	}
}

impl fmt::Debug for FileNode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	    f.debug_struct("FileNode")
		.field("val", &self.val)
		.field("left", &self.left)
		.finish()
	}
    }
