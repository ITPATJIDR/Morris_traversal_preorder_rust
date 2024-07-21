mod read_dir;
mod morris_traversal;

use std::cell::RefCell;
use std::rc::Rc;

use morris_traversal::read_file::{read_file, strip_prefix_from_paths};
use morris_traversal::{remote_duplicate, Node};

fn main() {
    // let folder_path = Path::new("/home/itpat/Code/Rust/modern-desktop-app-template");
    let path = "/Users/itpat/Code/Rust/Morris_traversal_preorder_rust/test.txt";
    let prefix = "/home/itpat/Code/Rust/modern-desktop-app-template/";
    let mut split_path:Vec<Vec<String>>= Vec::new();

    match read_file(path)  {
        Ok(paths) => {
            let split_contents: Vec<String> = paths 
                .split('\n')
                .map(|s| s.to_string())
                .collect();

            let strip_prefix:Vec<String>= strip_prefix_from_paths(split_contents, &prefix);

            for full_path in strip_prefix {
                let path: Vec<String> = full_path
                    .split('/')
                    .map(|s| s.to_string())
                    .collect();

                split_path.push(path)
            }

        } 
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    }

    let root:Rc<RefCell<Node>> = Node::new("/home");
    let left_child:Rc<RefCell<Node>> = Node::new("/test.js");
    let right_child:Rc<RefCell<Node>> = Node::new("/src");

    let x = remote_duplicate(split_path);

}
