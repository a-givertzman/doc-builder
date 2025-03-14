use std::{fs, path::{Path, PathBuf}};
use regex::Regex;
///
/// Scanning files & folders in the `path`
/// - builds nested collections of documents
/// - first md document in each folder should contains the header of it document
/// - following and nested documents will be joined to it header
#[derive(Clone)]
pub struct DocDir {
    pub path: PathBuf,
    pub children: Vec<DocDir>,
    pub is_dir: bool
}
//
//
impl DocDir {
    ///
    /// Returns [DocDir] built from files & folders found in the `path`
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
            children: vec![],
            is_dir: path.is_dir(),
        }
    }
    // ///
    // /// Appends a `path` to the back of a collection.
    // pub fn push(&mut self, path: DocDir) {
    //     self.children.push(path);
    //     self.children.sort_by(|dir_a, dir_b| dir_a.path.cmp(&dir_b.path));
    // }
    pub fn is_files_only(&self) -> bool {
        self.children.iter().all(|path| path.children.is_empty())
    }
    pub fn has_children(&self) -> bool {
        self.children.is_empty()
    }
    ///
    /// Returns string created from self.path as:  
    /// `part01_xyz` => `Part 01`
    pub fn header(&self) -> String {
        let re = Regex::new(r".*/(.\D+)(\d+)").unwrap();
        match re.captures(self.path.to_str().unwrap()) {
            Some(caps) => {
                format!(
                    "{} {}",
                    Self::uppercase_first_letter(
                        caps.get(1).map_or("???", |g| g.as_str()),
                    ),
                    caps.get(2).map_or("???", |g| g.as_str()),
                )
            }
            None => String::from("???"),
        }
    }
    ///
    /// Transform first char of string to upper case
    fn uppercase_first_letter(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
    ///
    /// Returns a list of all files & folders containing in the path
    /// - `ext` - filtering files by extension '.md' - fo example, empty mask - all files
    pub fn scan(mut self, ext: &str) -> DocDir {
        if self.path.is_dir() {
            match fs::read_dir(&self.path) {
                Ok(dirs) => {
                    for path in dirs.map(|d| d.unwrap().path()) {
                        if path.is_dir() {
                            self.children.push(DocDir::new(&path).scan(ext));
                        } else {
                            if ext.is_empty() {
                                self.children.push(DocDir::new(&path));
                            } else {
                                if let Some(path_ext) = path.extension() {
                                    if path_ext == ext {
                                        self.children.push(DocDir::new(&path));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => log::warn!("files | Error in path '{:?}': {:?}", self.path, err),
            }
        }
        self.children.sort_by(|path1, path2| path1.path.cmp(&path2.path));
        self
    }
}