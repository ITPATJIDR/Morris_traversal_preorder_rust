use std::collections::{HashMap};
use std::fs;
use std::io::Result;


struct dup_path {
    folder: Vec<String>,
    dup_file: Vec<String>,
}

pub fn read_file(path: &str) -> Result<String> {
    let contents: String = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn strip_prefix_from_paths(paths: Vec<String>, prefix: &str) -> Vec<String> {
    paths.into_iter()
        .filter_map(|path| path.strip_prefix(prefix).map(|s| s.to_string()))
        .collect()
}


pub fn remote_duplicate(split_path: Vec<Vec<String>>) -> (HashMap<Vec<String>, Vec<String>>, HashMap<usize, Vec<String>>) {
    let mut same_path: HashMap<usize, Vec<String>> = HashMap::new();
    let mut not_same_path: HashMap<usize, Vec<String>> = HashMap::new();
    let mut same_path_file: HashMap<Vec<String>, Vec<String>> = HashMap::new();

    let mut current_path: Vec<String> = Vec::with_capacity(1);
    let mut same_directory: Vec<String> = Vec::new();

    for i in 0..split_path.len() - 1 {
        if split_path[i].len() == split_path[i + 1].len() {
            same_path.insert(i, split_path[i].clone());
        } else {
            not_same_path.insert(i, split_path[i].clone());
        }
    }

    for i in 0..same_path.len() {
        if let Some(path) = same_path.get(&i) {
            let mut new_path: Vec<String> = path.clone();
            new_path.pop();

            if current_path.is_empty() {
                current_path = new_path;
            } else if current_path == new_path {
                if let Some(last_index) = path.last() {
                    same_directory.push(last_index.clone());
                }
            } else {
                same_path_file.insert(current_path.clone(), same_directory.clone());
                current_path = new_path;
                same_directory.clear();
            }
        }
    }

    if !current_path.is_empty() && !same_directory.is_empty() {
        same_path_file.insert(current_path, same_directory);
    }

    (same_path_file, not_same_path)
}
