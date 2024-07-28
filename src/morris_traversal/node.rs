use std::{ cell::{Ref, RefCell}, path, rc::Rc};

#[derive(Debug)]
pub struct Directory {
    directory_name: String,
    node_parent: Option<String>,
    next_node: Vec<Rc<RefCell<Directory>>>
}

#[derive(Debug)]
pub struct File{
    file_name: String,
    node_parent: Option<String>,
}

pub trait Node {
    fn new(path_name: String, node_parent: Option<String>) -> Rc<RefCell<Directory>>;
    fn get_name(&self) -> String;
    fn get_parent_node(&self) -> Option<String>;
}

pub trait OnlyDirectory {
    fn get_next_node(&self) -> Vec<Rc<RefCell<Directory>>>;
    fn add_new_child(&mut self, path_name: String, node_parent: Option<String>);
    fn contains_node(&self, path_name: String) -> bool;
}

impl Node for Directory {
    fn new(path_name: String, node_parent: Option<String>) -> Rc<RefCell<Directory>>{
        let new_directort_node = Rc::new(
            RefCell::new(
                Directory {
                    directory_name: path_name,
                    node_parent,
                    next_node: Vec::new()
                }
            )
        );

        new_directort_node
    }

    fn get_name(&self) -> String {
        self.directory_name.clone()
    }

    fn get_parent_node(&self) -> Option<String> {
        self.node_parent.clone()
    }


}

impl OnlyDirectory for Directory {
    fn get_next_node(&self) -> Vec<Rc<RefCell<Directory>>> {
        self.next_node.clone()
    }

    fn add_new_child(&mut self, path_name: String, node_parent: Option<String>){
        let new_node:Rc<RefCell<Directory>> = Directory::new(path_name.clone(), node_parent);
        if !self.contains_node(path_name.clone()){
            self.next_node.push(new_node);
        }
    }

    fn contains_node(&self, path_name: String) -> bool {
        self.next_node.iter().any(|item| item.borrow().get_name() == path_name)
    }
}

impl Node for File {
    fn new(path_name: String, node_parent: Option<String>) -> Rc<RefCell<Directory>>{
        let new_file_node = Rc::new(
            RefCell::new(
                Directory {
                    directory_name: path_name,
                    node_parent: node_parent,
                    next_node: Vec::new()
                }
            )
        );

        new_file_node
    }

    fn get_name(&self) -> String {
        self.file_name.clone()
    }

    fn get_parent_node(&self) -> Option<String> {
        self.node_parent.clone()
    }

}


