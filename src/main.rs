mod comrak_convert;
mod doc_dir;

use std::{ffi::OsString, fs, io::Write, path::{Component, Path, PathBuf}, process::Command};

use comrak_convert::ComrakConvert;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use doc_dir::DocDir;

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let path = PathBuf::from("sss/docs/user-guide/ru");
    let assets = PathBuf::from("sss/assets");
    let template = "template.html";
    ComrakConvert::new(&path, assets, template).convert();
    // let files = files(&path);
    // convert_markdown2html_converter(&path, files);
    // convert_markdown(files);
}
///
/// https://crates.io/crates/markdown2html-converter
fn convert_markdown2html_converter(path: &Path, files: DocDir) {
    let mut md_doc = String::new();
    for dir in files.children {
        if dir.is_files_only() {
            println!("\n{:?}", dir.path);
            for path in dir.children {
                let path = path.path;
                println!("\t{:?}", path);
                md_doc.push_str("\n");
                md_doc.push_str(
                    &fs::read_to_string(&path).unwrap(),
                );
            }
        } else {
            convert_markdown2html_converter(path, dir)
        }
    }
    let md_path = path.join("doc.md");
    let target_doc = "sss/doc.html";
    let mut file = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(&md_path)
        .unwrap();
    file.write_all(md_doc.as_bytes()).unwrap();
    Command::new("markdown2html-converter")
        .arg(&md_path)
        .arg("-f")
        .arg("-o")
        .arg(&target_doc)
        .output()
        .expect(&format!("failed to write '{:?}'", target_doc));
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
