use std::{fs, path::PathBuf};

fn main() {
    let files = [
        "/home/lobanov/code/rust/learn/md_to_pdf/docs/user-guide/ru/part03_mainScreen/chapter01_generalInfo.md",
    ];
    for path in files {
        let path = PathBuf::from(path);
        let markdown = fs::read_to_string(&path).unwrap();
        let file_name = PathBuf::from(path.file_stem().unwrap()).with_extension("pdf");
        // Convert with custom styling from markdown2pdfrc.toml
        markdown2pdf::parse(markdown, file_name.as_os_str().to_str().unwrap()).unwrap();
    }
}
