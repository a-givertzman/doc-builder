use std::fs;

use regex::Regex;

use super::{doc_dir::DocDir, title_page::Title};

///
/// Marcdown document
/// - Reads from file of folder
/// - Returns content
/// - Returns Title page
pub struct MdDoc {
    dir: DocDir,
    pub title: Option<Title>,
    pub body: String,
}
//
//
impl MdDoc {
    pub const PAGEBREAK: &str = "======================pagebreak======================";
    pub const BODY_CONTENT: &str = "======================body-section-content======================";
    /// Returns [Doc] new instance
    /// - `input` - string, contains markdown
    pub fn new(dir: DocDir) -> Self {
        Self {
            dir,
            title: None,
            body: String::new(),
        }
    }
    ///
    /// Returns marckdown document read from the specified `dir`
    /// - combined from multiple md files stored in the nested folders
    pub fn read(self) -> Self {
        log::debug!("Doc.read | path: '{:?}'", self.dir.path);
        let mut body = String::new();
        let mut title = None;
        Self::combine(&self.dir, &mut body, &mut title);
        let body = Self::add_pagebreakes(&body);
        Self {
            dir: self.dir,
            title,
            body
        }
    }
    ///
    /// Returns joined `title` and `body`
    pub fn joined(&self) -> String {
        format!("{}{}", self.title.clone().map_or("".into(), |t| t.raw), self.body)
    }
    /// 
    /// Add page brakes
    fn add_pagebreakes(doc: &str) -> String {
        let lines: Vec<&str> = doc.split("\n").collect();
        let mut doc = String::new();
        if let Some(line) = lines.first() {
            doc.push_str(line);
            doc.push_str("\n");
        }
        let mut prev_is_empty = false;
        let re_is_empty = Regex::new(r#"(^\s*$)"#).unwrap();
        for line in lines.into_iter().skip(1) {
            if line.starts_with("# ") {
                if !prev_is_empty {
                    doc.push_str("\n\n");
                }
                doc.push_str(MdDoc::PAGEBREAK);
                doc.push_str("\n\n");
            }
            doc.push_str(line);
            doc.push_str("\n");
            prev_is_empty = re_is_empty.is_match(line);
        }
        doc
    }
    ///
    /// Returns marckdown document
    /// combined from multiple md files stored in the nested folders
    fn combine(dir: &DocDir, body: &mut String, title: &mut Option<Title>) {
        log::debug!("Doc.combine | path: '{:?}'", dir.path);
        if !dir.is_dir {
            println!("\t{:?}", dir.path);
            if title.is_none() {
                if let Some(t) = Title::from(&dir.path) {
                    log::debug!("Doc.combine | Title: {:#?}", t);
                    *title = Some(t);
                };
            } else {
                body.push_str(
                    &fs::read_to_string(&dir.path).unwrap(),
                )
            }
        } else {
            body.push_str(&Self::read_header(&dir));
            let children = dir.children.iter().filter(|child| {
                if child.is_dir {
                    true
                } else {
                    child.header() != dir.header()
                }
            });
            for child in children {
                Self::combine(child, body, title)
            }
            if !body.ends_with("\n\n") {
                body.push_str("\n\n");
            }
            if !Self::ends_with_pagebreak(body) {
                body.push_str(Self::PAGEBREAK);
                body.push_str("\n\n");
            }
        }
    }
    ///
    /// Returns true if string has page break at the end
    fn ends_with_pagebreak(doc: &str) -> bool {
        let re_non_whitespace = Regex::new(r"\S").unwrap();
        let last_non_emty_line = doc
            .rsplit("\n")
            .skip_while(|line| !re_non_whitespace.is_match(line))
            .next();
        match last_non_emty_line {
            Some(last_line) => last_line.contains(Self::PAGEBREAK),
            None => false,
        }
    }
    ///
    /// Returns document, with repbuilded header:
    /// - take text from file path:
    /// 
    ///     `part01_xyz => Part 01`
    /// 
    /// - Rebuild document header as:
    /// 
    ///     `# Doc header => # Part 01. Doc header`
    fn rebuild_header(doc: &DocDir, ) -> String {
        let lines = fs::read_to_string(&doc.path).unwrap();
        let mut lines: Vec<&str> = lines.split('\n').collect();
        let re = Regex::new(r"^[ \t]*(#*)[ \t](.*)$").unwrap();
        let first_line = lines.remove(0);
        let first_line = match re.captures(first_line) {
            Some(caps) => format!(
                "{} {}. {}\n\n",
                caps.get(1).map_or("???", |g| g.as_str()),
                doc.header(),
                caps.get(2).map_or("???", |g| g.as_str()),
            ),
            None => first_line.to_owned(),
        };
        let content = if lines.len() > 1 {
            lines.join("\n")
        } else {
            "\n\n".to_owned()
        };
        format!("{}{}", first_line, content)
    }
    ///
    /// Reads and returns header document 
    /// from first found child contains header in format `^#* `
    /// - Returns empty string if header is not found
    fn read_header(dir: &DocDir) -> String {
        let first = dir.children.iter().find(|child| {
            (!child.is_dir) && child.header() == dir.header()
        });
        match first {
            Some(first) => Self::rebuild_header(first),
            None => {
                log::warn!("read_header | Header not found in '{:?}'", dir.path);
                String::new()
            },
        }
    }
}
