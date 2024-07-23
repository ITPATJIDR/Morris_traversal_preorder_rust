use core::{fmt, str};
use std::borrow::{Borrow, BorrowMut};
use std::path;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub struct Node {
    pub name: String,
    pub node_list: Vec<Option<Rc<RefCell<Node>>>>,
    pub is_directory: bool,
}

impl Node {
    pub fn new(name: &str, is_directory: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            node_list: Vec::new(),
            is_directory,
        }))
    }

    pub fn add_node(&mut self, name: &str, is_directory: bool) {
        let new_node = Node::new(name, is_directory);
        self.node_list.push(Some(new_node));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_node_list(&self) -> &Vec<Option<Rc<RefCell<Node>>>> {
        &self.node_list
    }

    pub fn get_is_directory(&self) -> &bool {
        &self.is_directory
    }
}

#[derive(Debug)]
pub struct Tree {
    root: Option<Rc<RefCell<Node>>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn get_root(&self) -> Option<Rc<RefCell<Node>>> {
        self.root.clone()
    }

    pub fn insert(&mut self, path_name: &str, is_directory: bool) {
        match &self.root {
            Some(root) => {
		println!("{:?}", &root)
            }
            None => {
                self.root = Some(Node::new(path_name, is_directory));
            }
        }
    }
}



impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	    f.debug_struct("DirectoryNode")
		.field("val", &self.name)
		.field("node_list", &self.node_list)
		.field("node_type", &self.is_directory)
		// Add more fields as needed
		.finish()
	}
}
