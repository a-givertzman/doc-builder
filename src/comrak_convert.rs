use std::{fs, io::Write, path::{Path, PathBuf}};

use regex::Regex;

use crate::doc_dir::DocDir;

///
/// Converts multiple `markdown` documents into the single `Html`
/// 
/// Based on:
/// https://github.com/kivikakk/comrak
pub struct ComrakConvert {
    path: PathBuf,
    assets: PathBuf,
    template: PathBuf,
}
//
//
impl ComrakConvert {
    ///
    /// Returns ComracConvert new instance
    /// - `path` - folder with markdown documents
    /// - `assets` - folder with asset files
    pub fn new(path: impl AsRef<Path>, assets: impl AsRef<Path>, template: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            assets: assets.as_ref().to_path_buf(),
            template: template.as_ref().to_path_buf(),
        }
    }
    ///
    /// Performs a conversion
    pub fn convert(&self) {
        let dir = DocDir::new(&self.path).scan();
        let mut doc = String::new();
        Self::combine(dir.clone(), &mut doc);
        let target = self.assets.parent().unwrap().join("doc.html");
        let md_path = self.assets.parent().unwrap().join("doc.md");
        let mut file = fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&md_path)
            .unwrap();
        file.write_all(doc.as_bytes()).unwrap();
        let html = Self::comrack_parse(&doc);
        let template = fs::read_to_string(&self.template).unwrap();
        let html = template.replace("content", &html);
        let html = html.replace("======================pagebreak======================", "<div class=\"pagebreak\"> </div>");
        let mut file = fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(target)
            .unwrap();
        file.write_all(html.as_bytes()).unwrap();
    
    }
    ///
    /// Returns marckdown `document` combined from md files
    fn combine(dir: DocDir, doc: &mut String) {
        println!("\n{:?}", dir.path);
        let first = dir.children.iter().find(|child| {
            (!child.is_dir) && child.header() == dir.header()
        });
        match first {
            Some(first) => {
                let lines = fs::read_to_string(&first.path).unwrap();
                let mut lines: Vec<&str> = lines.split('\n').collect();
                let re = Regex::new(r"^[ \t]*(#*)[ \t](.*)$").unwrap();
                let first_line = lines.remove(0);
                let first_line = match re.captures(first_line) {
                    Some(caps) => format!(
                        "{} {}. {}\n\n",
                        caps.get(1).map_or("???", |g| g.as_str()),
                        first.header(),
                        caps.get(2).map_or("???", |g| g.as_str()),
                    ),
                    None => first_line.to_owned(),
                };
                let content = if lines.len() > 1 {
                    lines.join("\n")
                } else {
                    "\n\n".to_owned()
                };
                doc.extend([
                    first_line,
                    content,
                ]);
            }
            None => {
                log::warn!("convert_comrack | Headeer not found in '{:?}'", dir.path);
            },
        }
    
        let children = dir.children.iter().filter(|child| {
            if child.is_dir {
                true
            } else {
                child.header() != dir.header()
            }
        });
        for child in children {
            if child.is_dir {
                Self::combine(child.to_owned(), doc)
            } else {
                println!("\t{:?}", child.path);
                doc.push_str(
                    &fs::read_to_string(&child.path).unwrap(),
                );
            }
            doc.push_str("\n\n");
        }
        doc.push_str("\n\n");
        doc.push_str("======================pagebreak======================");
        doc.push_str("\n\n");
    }
    ///
    /// Returns a `html` representation of the markdown `document`
    fn comrack_parse(document: &str) -> String {
        // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
        let arena = comrak::Arena::new();
        // Parse the document into a root `AstNode`
        let root = comrak::parse_document(
            &arena,
            document,
            &comrak::Options {
                extension: comrak::ExtensionOptions::builder()
                    .strikethrough(true)
                    .tagfilter(true)
                    .table(true)
                    .autolink(true)
                    .tasklist(true)
                    .superscript(true)
                    // .header_ids(true)
                    .footnotes(true)
                    .description_lists(true)
                    // .front_matter_delimiter(true)
                    .multiline_block_quotes(true)
                    .math_dollars(true)
                    .math_code(true)
                    .wikilinks_title_after_pipe(true)
                    .wikilinks_title_before_pipe(true)
                    .underline(true)
                    .subscript(true)
                    .spoiler(true)
                    .greentext(true)
                    // .image_url_rewriter(true)
                    // .link_url_rewriter(true)
                    .build(),
                parse: comrak::ParseOptions::builder()
                    .smart(true)
                    // .default_info_string()
                    // .relaxed_tasklist_matching()
                    // .relaxed_autolinks()
                    // .broken_link_callback()
                    .build(),
                render: comrak::RenderOptions::builder()
                    // .hardbreaks()
                    // .github_pre_lang()
                    // .full_info_string()
                    // .width()
                    .unsafe_(true)
                    // .escape()
                    // .list_style()
                    // .sourcepos()
                    // .experimental_inline_sourcepos()
                    // .escaped_char_spans()
                    // .ignore_setext()
                    // .ignore_empty_links()
                    // .gfm_quirks()
                    // .prefer_fenced()
                    // .figure_with_caption()
                    // .tasklist_classes()
                    // .ol_width()
                    .build()
            },
        );
        // Iterate over all the descendants of root.
        // for node in root.descendants() {
        //     if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
        //         // If the node is a text node, replace `orig_string` with `replacement`.
        //         *text = text.replace(orig_string, replacement)
        //     }
        // }
        let mut html = vec![];
        comrak::format_html(root, &comrak::Options::default(), &mut html).unwrap();
        String::from_utf8(html).unwrap()
    }
}