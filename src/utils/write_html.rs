use std::{io::Write, path::{Path, PathBuf}};

use super::{eval::Eval, md_doc::MdDoc};

///
/// Writes input md into file with te specified path
/// - Returns input md, not modified
pub struct WriteHtml {
    path: PathBuf,
    input: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl WriteHtml {
    ///
    /// Returns [WriteHtml] new instance
    pub fn new(path: &Path, input: impl Eval<(), MdDoc> +'static) -> Self {
        Self {
            path: path.to_owned(),
            input: Box::new(input),
        }
    }
}
//
//
impl Eval<(), MdDoc> for WriteHtml {
    /// Writes input md into file with te specified path
    /// - Returns input md, not modified
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        let mut file = std::fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&self.path)
            .unwrap();
        file.write_all(doc.html.as_bytes()).unwrap();
        doc
    }
}