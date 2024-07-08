mod read_dir;
mod morris_traversal;

use morris_traversal::read_file::{read_file, strip_prefix_from_paths};

fn main() {
    // let folder_path = Path::new("/home/itpat/Code/Rust/modern-desktop-app-template");

    let path = "/Users/itpat/Code/Morris_traversal_preorder_rust/test.txt";
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

    println!("{:?}",split_path)
    // match read_dir_recursive(folder_path) {
    //     Ok(entries) => {
    //         for entry in entries {
    //             println!("{}", entry);
    //         }
    //     }
    //     Err(err) => {
    //         eprintln!("Error reading directory: {}", err);
    //     }
    // }
}
