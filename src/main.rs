use std::{ffi::OsString, fs, io::Write, path::{Component, Path, PathBuf}};

fn main() {
    // let files = [
    //     "/home/lobanov/code/rust/learn/md_to_pdf/docs/user-guide/ru/part03_mainScreen/chapter01_generalInfo.md",
    //     "/home/lobanov/code/rust/learn/md_to_pdf/docs/user-guide/ru/part03_mainScreen/chapter02_navigationPanel.md",
    //     "/home/lobanov/code/rust/learn/md_to_pdf/docs/user-guide/ru/part03_mainScreen/chapter03_draftPicture.md",
    // ];
    let files = files(&PathBuf::from("sss/docs/user-guide/ru"));
    for path in files {
        println!("{:?}", path);
        let md_contents = fs::read_to_string(&path).unwrap();
        let mut target_path: Vec<_> = path
            .parent()
            .unwrap()
            .components()
            .collect();
        let prefix = OsString::from("docs_target");
        target_path[0] = Component::Normal(&prefix);
        let target_path: PathBuf = target_path.into_iter().collect();
        let target = PathBuf::from(target_path.with_extension("pdf"));
        // // Convert with custom styling from markdown2pdfrc.toml
        // markdown2pdf::parse(markdown, file_name.as_os_str().to_str().unwrap()).unwrap();
        let html = markdown::to_html(&md_contents);
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

fn files(path: &Path) -> Vec<PathBuf> {
    let mut result = vec![];
    match fs::read_dir(path) {
        Ok(dirs) => {
            for path in dirs.map(|d| d.unwrap().path()) {
                if path.is_dir() {
                    result.extend(files(&path));
                }
                if path.is_file() {
                    result.push(path);
                }
            }
        }
        Err(err) => println!("files | Error in path '{:?}': {:?}", path, err),
    }
    result
}