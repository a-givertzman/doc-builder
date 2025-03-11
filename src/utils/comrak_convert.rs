use std::{fs::{self}, io::{BufReader, Read, Write}, path::{Path, PathBuf}, sync::Arc};

// use base64::{engine::general_purpose, Engine};
// use image::{DynamicImage, ImageFormat};
use regex::Regex;
use crate::utils::title_page::Title;

use super::{doc_dir::DocDir, md_doc::MdDoc};
// use crate::doc_dir::DocDir;

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
    // math_script: PathBuf,
}
//
//
impl ComrakConvert {
    // const MATH_MODULE: &str = "======================math-module======================";
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
            // math_script: PathBuf::from("src/mathJax/es5/tex-mml-chtml.js"),
        }
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
                // let path_str = path.as_str();
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
    // ///
    // /// 
    // fn image_to_base64(img: &DynamicImage) -> String {
    //     let mut image_data: Vec<u8> = Vec::new();
    //     img.write_to(&mut std::io::Cursor::new(&mut image_data), ImageFormat::Png)
    //         .unwrap();
    //     let res_base64 = general_purpose::STANDARD.encode(image_data);
    //     format!("data:image/png;base64,{}", res_base64)
    // }
    // ///
    // /// Embedding formula math module js script
    // fn embedd_math(html: &str, path: &Path) -> String {
    //     let script = fs::read_to_string(path).unwrap();
    //     let math_re = format!(r"[ \t]*//[ \t]*{}.*", Self::MATH_MODULE);
    //     log::debug!("embedd_math | math_module: {}", script.len());
    //     log::debug!("embedd_math | math_re: '{}'", math_re);
    //     let re = Regex::new(&math_re).unwrap();
    //     let html = re.replace(html, script).into_owned();
    //     // log::debug!("embedd_math | html: '{}'", html);
    //     html
    // }
    ///
    /// Performs a conversion
    pub fn convert(&self) {
        let target = if self.output.is_dir() {
            self.output.join("doc.html")
        } else {
            self.output.with_extension("html")
        };
        let dir = DocDir::new(&self.path).scan("md");
        let doc = MdDoc::new(dir).read();
        // Self::combine(dir.clone(), &mut doc);
        // doc = Self::add_pagebreakes(&doc);
        let md_path = self.output.with_extension("md");
        let mut file = fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&md_path)
            .unwrap();
        file.write_all(doc.joined().as_bytes()).unwrap();

        let html = Self::comrack_parse(&doc.body);
        let html = Self::embedd_images(&html, &self.assets);
        let html = match fs::read_to_string(&self.template) {
            Ok(template) => {
                // let template = Self::embedd_math(&template, &self.math_script);
                template.replace(MdDoc::BODY_CONTENT, &html)
            }
            Err(_) => {
                log::debug!("convert | Default template.html - is not found in: {:?}", self.template.as_os_str());
                html
            }
        };
        let html = match &doc.title {
            Some(title) => {
                log::debug!(".convert | Title page: {:#?}", title);
                let html = html.replace(Title::LOGO, &title.logo);
                let html = html.replace(Title::ADDR, &title.addr);
                let html = html.replace(Title::NAME, &title.name);
                let html = html.replace(Title::DESCR, &title.descr);
                html
            }
            None => {
                log::warn!(".convert | Title page not found");
                html
            }
        };
        let html = html.replace(MdDoc::PAGEBREAK, "<div class=\"pagebreak\"> </div>");
        let mut file = fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(target)
            .unwrap();
        file.write_all(html.as_bytes()).unwrap();
    
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
