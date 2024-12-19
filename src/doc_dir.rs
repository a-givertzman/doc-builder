use std::path::{Path, PathBuf};

use regex::Regex;

///
/// 
#[derive(Clone)]
pub struct DocDir {
    pub path: PathBuf,
    pub children: Vec<DocDir>,
}
//
//
impl DocDir {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
            children: vec![],
        }
    }
    pub fn push(&mut self, path: DocDir) {
        self.children.push(path);
        self.children.sort_by(|dir_a, dir_b| dir_a.path.cmp(&dir_b.path));
    }
    pub fn is_files_only(&self) -> bool {
        self.children.iter().all(|path| path.children.is_empty())
    }
    pub fn has_children(&self) -> bool {
        self.children.is_empty()
    }
    pub fn header(&self) -> String {
        let re = Regex::new(r".*/(.\D+)(\d+)").unwrap();
        match re.captures(self.path.to_str().unwrap()) {
            Some(caps) => {
                format!(
                    "{} {}",
                    caps.get(1).map_or("???", |g| g.as_str()),
                    caps.get(2).map_or("???", |g| g.as_str()),
                )
            }
            None => String::from("???"),
        }
    }
}