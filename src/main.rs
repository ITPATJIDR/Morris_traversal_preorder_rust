mod read_dir;
mod morris_traversal;
use std::{borrow::{Borrow, BorrowMut}, collections::HashMap, vec};
use morris_traversal::{read_file::{read_file, strip_prefix_from_paths},tree::{Directory, Tree}};

fn main() {
    // let folder_path = Path::new("/home/itpat/Code/Rust/modern-desktop-app-template");
    // let path = "/Users/itpat/Code/Rust/Morris_traversal_preorder_rust/test.txt";
    let path = "/home/itpat/Code/Rust/Morris_traversal_preorder/test.txt";
    let prefix = "/home/itpat/Code/Rust/modern-desktop-app-template/";
    let mut strip_prefix: Vec<String> = Vec::new();

    match read_file(path)  {
        Ok(paths) => {
            let split_contents: Vec<String> = paths 
                .split('\n')
                .map(|s| s.to_string())
                .collect();

            strip_prefix = strip_prefix_from_paths(split_contents, &prefix);
        } 
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    }

    let mut root = Directory::new("/HI");

    for path in strip_prefix {
        let parts: Vec<&str> = path.split('/').collect();
        root.add_path(&parts[0..])
    }

    root.print(0);
}
