use std::{borrow::BorrowMut, collections::HashMap};

#[derive(Debug)]
pub struct Directory {
    name: String,
    subdirs: HashMap<String, Directory>,
    files: Vec<String>,
}

pub trait Tree {
    fn new(name:&str) -> Self;
    fn add_path(&mut self, parts: &[&str]);
    fn print(&self, level: usize);
}

impl Tree for Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subdirs: HashMap::new(),
            files: Vec::new(),
        }
    }

    fn add_path(&mut self, parts: &[&str]) {
        if parts.len() == 1 {
            self.files.push(parts[0].to_string());
        } else {
            let dir = self.subdirs.entry(parts[0].to_string()).or_insert_with(|| Directory::new(parts[0]));
            dir.add_path(&parts[1..]);
        }
    }

    fn print(&self, level: usize) {
        let indent = "  ".repeat(level);
        if level > 0 {
            println!("{}- /{}", indent, self.name);
        }
        for file in &self.files {
            println!("{}  - {}", indent, file);
        }
        for subdir in self.subdirs.values() {
            subdir.print(level + 1);
        }
    }
}



