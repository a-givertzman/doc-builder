use std::{fs::{self, File}, io::{BufRead, BufReader, Read, Write}, path::{Path, PathBuf}, sync::Arc};

use base64::{engine::general_purpose, Engine};
use image::{DynamicImage, ImageFormat};
use regex::Regex;

use crate::doc_dir::DocDir;

///
/// Converts multiple `markdown` documents into the single `Html`
/// 
/// Based on:
/// https://github.com/kivikakk/comrak
pub struct ComrakConvert {
    path: PathBuf,
    output: PathBuf,
    assets: PathBuf,
    template: PathBuf,
    math_script: PathBuf,
}
//
//
impl ComrakConvert {
    const CONTENT: &str = "======================content======================";
    const PAGEBREAK: &str = "======================pagebreak======================";
    const MATH_MODULE: &str = "======================math-module======================";
    ///
    /// Returns ComracConvert new instance
    /// - `path` - folder with markdown documents
    /// - `assets` - folder with asset files
    pub fn new(path: impl AsRef<Path>, output: impl AsRef<Path>, assets: impl AsRef<Path>, template: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            output: output.as_ref().to_path_buf(),
            assets: assets.as_ref().parent().unwrap_or(assets.as_ref()) .to_path_buf(),
            template: template.as_ref().to_path_buf(),
            math_script: PathBuf::from("src/mathJax/es5/tex-mml-chtml.js"),
        }
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
                doc.push_str(Self::PAGEBREAK);
                doc.push_str("\n\n");
            }
            doc.push_str(line);
            doc.push_str("\n");
            prev_is_empty = re_is_empty.is_match(line);
        }
        doc
    }
    ///
    /// Embedding images into Html
    fn embedd_images(html: &str, assets: &Path) -> String {
        let mut result = String::new();
        let re = Regex::new(r#"(<img\s+?src=")(.*?)(".*?/>)"#).unwrap();
        let mut las_match = 0;
        for item in re.captures_iter(html) {
            // log::debug!("embedd_images | img: {:?}", item.get(2));
            if let (Some(prefix), Some(path), Some(sufix)) = (item.get(1), item.get(2), item.get(3)) {
                result.push_str(
                    &html[las_match..prefix.start()]
                );
                let path_str = path.as_str();
                let path = if path.as_str().starts_with("/") {
                    assets.join(path.as_str().trim_start_matches("/"))
                } else {
                    assets.join(path.as_str())
                };
                if let Some(ext) = path.extension().and_then(std::ffi::OsStr::to_str) {
                    match ext {
                        "svg" => {
                            log::debug!("embedd_images | SVG img: {:?}", path);
                            match std::fs::File::open(&path) {
                                Ok(file) => {
                                    let mut img = String::new();
                                    match BufReader::new(file).read_to_string(&mut img) {
                                        Ok(_) => {
                                            result.push_str(&img);
                                        }
                                        Err(err) => {
                                            log::warn!("embedd_images | Error read img file: '{:?}': \n\t{:?}", path, err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    log::warn!("embedd_images | Error acces img file: '{:?}': \n\t{:?}", path, err);
                                }
                            }
                        }
                        _ => {
                            log::debug!("embedd_images | img by ref: {:?}...", path);
                            //     let img = image::ImageReader::open(&path).unwrap().decode().unwrap();
                            //     log::debug!("embedd_images | reading img: {:?} - Ok", path);
                            //     let img = Self::image_to_base64(&img);
                            //     let img = format!("{}{}{}", prefix.as_str(), img, sufix.as_str());
                            //     // <img src="">
                            result.push_str(prefix.as_str());
                            result.push_str(path.as_os_str().to_str().unwrap());
                            result.push_str(sufix.as_str());
                        }
                    }
                }
                las_match = sufix.end();
            }
        }
        result.push_str(
            &html[las_match..]
        );
        result
        // html.to_owned()
    }
    ///
    /// 
    fn image_to_base64(img: &DynamicImage) -> String {
        let mut image_data: Vec<u8> = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut image_data), ImageFormat::Png)
            .unwrap();
        let res_base64 = general_purpose::STANDARD.encode(image_data);
        format!("data:image/png;base64,{}", res_base64)
    }
    ///
    /// Embedding formula math module js script
    fn embedd_math(html: &str, path: &Path) -> String {
        let script = fs::read_to_string(path).unwrap();
        let math_re = format!(r"[ \t]*//[ \t]*{}.*", Self::MATH_MODULE);
        log::debug!("embedd_math | math_module: {}", script.len());
        log::debug!("embedd_math | math_re: '{}'", math_re);
        let re = Regex::new(&math_re).unwrap();
        let html = re.replace(html, script).into_owned();
        // log::debug!("embedd_math | html: '{}'", html);
        html
    }
    ///
    /// Performs a conversion
    pub fn convert(&self) {
        let mut doc = String::new();
        let target = if self.output.is_dir() {
            self.output.join("doc.html")
        } else {
            self.output.with_extension("html")
        };
        if self.path.is_dir() {
            let dir = DocDir::new(&self.path).scan("md");
            Self::combine(dir.clone(), &mut doc);
            doc = Self::add_pagebreakes(&doc);
            let md_path = self.output.with_extension("md");
            let mut file = fs::OpenOptions::new()
                .truncate(true)
                .create(true)
                .write(true)
                .open(&md_path)
                .unwrap();
            file.write_all(doc.as_bytes()).unwrap();
        } else {
            let mut file = fs::OpenOptions::new()
                .read(true)
                .open(&self.path)
                .unwrap();
            file.read_to_string(&mut doc).unwrap();
            doc = Self::add_pagebreakes(&doc);
        };
        let html = Self::comrack_parse(&doc);
        let html = Self::embedd_images(&html, &self.assets);
        let html = match fs::read_to_string(&self.template) {
            Ok(template) => {
                // let template = Self::embedd_math(&template, &self.math_script);
                template.replace(Self::CONTENT, &html)
            }
            Err(_) => {
                log::debug!("convert | Default template.html - is not found in: {:?}", self.template.as_os_str());
                html
            }
        };
        let html = html.replace(Self::PAGEBREAK, "<div class=\"pagebreak\"> </div>");
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
        log::debug!("\npath: '{:?}'", dir.path);
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
        if !doc.ends_with("\n\n") {
            doc.push_str("\n\n");
        }
        if !Self::ends_with_pagebreak(doc) {
            doc.push_str(Self::PAGEBREAK);
            doc.push_str("\n\n");
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
                    // .math_dollars(true)
                    // .math_code(true)
                    .wikilinks_title_after_pipe(true)
                    .wikilinks_title_before_pipe(true)
                    .underline(true)
                    .subscript(true)
                    .spoiler(true)
                    .greentext(true)
                    .image_url_rewriter(Arc::new(|url: &str| {
                        log::debug!("url: {}", url);
                        // format!("https://safe.example.com?url={}", url)
                        url.to_owned()
                    }))
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