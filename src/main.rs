use std::path::Path;
mod read_dir;

use read_dir::read_dir_recursive;

fn main() {
    let folder_path = Path::new("/home/itpat/Code/Rust/modern-desktop-app-template");
    match read_dir_recursive(folder_path) {
        Ok(entries) => {
            for entry in entries {
                println!("{}", entry);
            }
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    }
}
