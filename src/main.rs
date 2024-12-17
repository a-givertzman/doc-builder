use std::{ffi::OsString, fs, io::Write, path::{Component, Path, PathBuf}, process::Command};

use comrak::html;

fn main() {
    let files = files(&PathBuf::from("sss/docs/user-guide/ru"));
    convert_comrack(files);
    // convert_markdown2html_converter(files);
    // convert_markdown(files);
}
///
/// https://crates.io/crates/markdown2html-converter
fn convert_markdown2html_converter(files: Vec<DocDir>) {
    for dir in files {
        if dir.is_files_only() {
            for path in dir.children {
                let path = path.path;
                println!("\n{:?}", path);
                let mut target_path: Vec<_> = path
                    .parent().unwrap()
                    // .parent().unwrap()
                    .components()
                    .collect();
                let prefix = OsString::from("docs_target");
                target_path[0] = Component::Normal(&prefix);
                let target_path: PathBuf = target_path.into_iter().collect();
                let target = PathBuf::from(target_path.join(path.file_stem().unwrap()).with_extension("html"));
                if !target_path.exists() {
                    println!("creating path: {:?}", target_path);
                    fs::create_dir_all(target_path).unwrap();
                }
                Command::new("markdown2html-converter")
                    .arg(&path)
                    .arg("-f")
                    .arg("-o")
                    .arg(&target)
                    .output()
                    .expect(&format!("failed to write '{:?}'", target));
            }
        } else {
            convert_markdown2html_converter(dir.children)
        }
    }
}
///
/// https://github.com/kivikakk/comrak
fn convert_comrack(files: DocDir) {
    for dir in files.children {
        if dir.is_files_only() {
            let mut md_doc = String::new();
            println!("\n{:?}", dir.path);
            for path in dir.children {
                let path = path.path;
                println!("\t{:?}", path);
                md_doc.push_str(
                    &fs::read_to_string(&path).unwrap(),
                );
            }
            let mut target_path: Vec<_> = dir.path
                .components()
                .collect();
            let prefix = OsString::from("docs_target");
            target_path[0] = Component::Normal(&prefix);
            let name = target_path.last().unwrap().clone();
            let target_path: PathBuf = target_path.into_iter().collect();
            let target = PathBuf::from(target_path.join(name).with_extension("html"));
            if !target_path.exists() {
                println!("creating path: {:?}", target_path);
                fs::create_dir_all(target_path).unwrap();
            }
            println!("       target: {:?}", target);
            let html = comrack_parse(&md_doc, "", "");
            let mut file = fs::OpenOptions::new()
                .truncate(true)
                .create(true)
                .write(true)
                .open(target)
                .unwrap();
            file.write_all(html.as_bytes()).unwrap();
        } else {
            convert_comrack(dir)
        }
    }
}
fn comrack_parse(document: &str, orig_string: &str, replacement: &str) -> String {
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
                // .unsafe_()
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
///
/// https://github.com/wooorm/markdown-rs
fn convert_markdown(files: Vec<PathBuf>) {
    for path in files {
        println!("\n{:?}", path);
        let md_contents = fs::read_to_string(&path).unwrap();
        let mut target_path: Vec<_> = path
            .parent().unwrap()
            // .parent().unwrap()
            .components()
            .collect();
        let prefix = OsString::from("docs_target");
        target_path[0] = Component::Normal(&prefix);
        let target_path: PathBuf = target_path.into_iter().collect();
        let target = PathBuf::from(target_path.join(path.file_stem().unwrap()).with_extension("html"));
        println!("target: {:?}", target_path);
        println!("target: {:?}", target);
        // // // Convert with custom styling from markdown2pdfrc.toml
        // // markdown2pdf::parse(markdown, file_name.as_os_str().to_str().unwrap()).unwrap();
        let html = markdown::to_html_with_options(
            &md_contents,
            &markdown::Options {
                parse: markdown::ParseOptions {
                    constructs: markdown::Constructs {
                        attention: true,
                        autolink: true,
                        block_quote: true,
                        character_escape: true,
                        character_reference: true,
                        code_indented: true,
                        code_fenced: true,
                        code_text:true,
                        definition: true,
                        frontmatter: true,
                        gfm_autolink_literal: true,
                        gfm_footnote_definition: true,
                        gfm_label_start_footnote: true,
                        gfm_strikethrough: true,
                        gfm_table: true,
                        gfm_task_list_item: true,
                        hard_break_escape: true,
                        hard_break_trailing: true,
                        heading_atx: true,
                        heading_setext: true,
                        html_flow: true,
                        html_text: true,
                        label_start_image: true,
                        label_start_link: true,
                        label_end: true,
                        list_item: true,
                        math_flow: true,
                        math_text: true,
                        mdx_esm: true,
                        mdx_expression_flow: true,
                        mdx_expression_text: true,
                        mdx_jsx_flow: true,
                        mdx_jsx_text: true,
                        thematic_break: true,
                    },
                    gfm_strikethrough_single_tilde: true,
                    math_text_single_dollar: true,
                    mdx_expression_parse: Default::default(),
                    mdx_esm_parse: Default::default(), 
                },
                compile: markdown::CompileOptions { 
                    allow_dangerous_html: true, 
                    allow_dangerous_protocol: true, 
                    default_line_ending: markdown::LineEnding::default(), 
                    gfm_footnote_label: Default::default(), 
                    gfm_footnote_label_tag_name: Default::default(), 
                    gfm_footnote_label_attributes: Default::default(), 
                    gfm_footnote_back_label: Default::default(), 
                    gfm_footnote_clobber_prefix: Default::default(), 
                    gfm_task_list_item_checkable: Default::default(), 
                    gfm_tagfilter: Default::default(),
                },
            }
        ).unwrap();
        // let html = "<h1>Hello, world!</h1>";
        if !target_path.exists() {
            println!("creating path: {:?}", target_path);
            fs::create_dir_all(target_path).unwrap();
        }
        let mut file = fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(target)
            .unwrap();
        file.write_all(html.as_bytes()).unwrap();
    }
}


fn files(path: &Path) -> DocDir {
    let mut result = DocDir::new(&path);
    match fs::read_dir(path) {
        Ok(dirs) => {
            for path in dirs.map(|d| d.unwrap().path()) {
                if path.is_dir() {
                    result.push(
                        files(&path),
                    );
                } else {
                    result.push(DocDir::new(&path));
                }
            }
        }
        Err(err) => println!("files | Error in path '{:?}': {:?}", path, err),
    }
    result
}
///
/// 
struct DocDir {
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
}