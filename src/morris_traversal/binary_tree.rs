use core::{fmt, str};
use std::path;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

// Node structure
pub struct Node {
    pub name: String,
    pub node_list: Vec<Option<Rc<RefCell<Node>>>>,
    pub is_directory: bool,
    pub prev_node: Option<Rc<RefCell<Node>>>
}

trait Modnode {
    fn new(name: &str, is_directory: bool) -> Rc<RefCell<Self>>;
    fn add_node(&mut self, name: &str, is_directory: bool);
    fn get_name(&self) -> &str;
    fn get_node_list(&self) -> &Vec<Option<Rc<RefCell<Node>>>>;
    fn get_is_directory(&self) -> &bool;
    fn contains_node(&self, name: &str) -> bool;
    fn get_prev_node(&self) -> &Option<Rc<RefCell<Node>>>;
}

impl Modnode for Node {
    fn new(name: &str, is_directory: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            node_list: Vec::new(),
            is_directory,
            prev_node: None
        }))
    }

    fn add_node(&mut self, name: &str, is_directory: bool) {
        let new_node = Node::new(name, is_directory);
        if !self.contains_node(name) {
            self.node_list.push(Some(new_node));
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_list(&self) -> &Vec<Option<Rc<RefCell<Node>>>> {
        &self.node_list
    }

    fn get_is_directory(&self) -> &bool {
        &self.is_directory
    }

    fn get_prev_node(&self) -> &Option<Rc<RefCell<Node>>> {
        &self.prev_node
    }

    fn contains_node(&self, name: &str) -> bool {
        self.node_list.iter().any(|node_option| {
            if let Some(node_rc) = node_option {
                node_rc.borrow().get_name() == name
            } else {
                false
            }
        })
    }
}

// Tree structure
#[derive(Debug)]
pub struct Tree {
    root: Option<Rc<RefCell<Node>>>,
}

// Implementation of Tree
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
                root.borrow_mut().add_node(path_name, is_directory)
            },
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
		.field("prev_node", &self.prev_node)
		// Add more fields as needed
		.finish()
	}
}
