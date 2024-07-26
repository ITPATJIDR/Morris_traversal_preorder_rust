mod read_dir;
mod morris_traversal;

use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use morris_traversal::binary_tree::Tree;
use morris_traversal::read_file::{read_file, strip_prefix_from_paths};
use morris_traversal::{remote_duplicate, Node};

fn main() {
    // let folder_path = Path::new("/home/itpat/Code/Rust/modern-desktop-app-template");
    // let path = "/Users/itpat/Code/Rust/Morris_traversal_preorder_rust/test.txt";
    let path = "/home/itpat/Code/Rust/Morris_traversal_preorder/test.txt";
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

    let mut root:Tree = Tree::new();
    root.insert(&prefix, false);

    let (same_path_file,not_same_path_file):(HashMap<Vec<String>, Vec<String>>, HashMap<usize, Vec<String>>)= remote_duplicate(split_path);

    for (path, file) in same_path_file.iter() {
        for item in 0..path.len() {
            println!("{}",path[item]);
            if item == 0 {
                root.insert(&path[item], false);
            }
        }
        // println!("{}","--------------------------------------------------------");
        // for item in 0..file.len() {
        //     println!("{}",file[item])
        // }
        // println!("{}","========================================================");
    }

    println!("{:?}", root.get_root())


    // println!("Same path file : {:?}, ", same_path_file.len());
    // println!("Not Same path file : {:?}, ", not_same_path_file.len());
    // println!("Sum : {:?}", (same_path_file.len() + not_same_path_file.len()))

}
