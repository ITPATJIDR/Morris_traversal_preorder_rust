use std::fs;
use std::io::Result;

pub fn read_file(path: &str) -> Result<String> {
    let contents: String = fs::read_to_string(path)?;

    Ok(contents)
}
    // let stripped_contents = strip_prefix_from_paths(contents, prefix);

    // let split_contents: Vec<String> = contents
    //     .split('/')
    //     .map(|s| s.to_string())
    //     .collect();


pub fn strip_prefix_from_paths(paths: Vec<String>, prefix: &str) -> Vec<String> {
    paths.into_iter()
        .filter_map(|path| path.strip_prefix(prefix).map(|s| s.to_string()))
        .collect()
}