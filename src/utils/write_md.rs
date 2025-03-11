use std::{io::Write, path::{Path, PathBuf}};

use super::{eval::Eval, md_doc::MdDoc};

///
/// Writes input md into file with te specified path
/// - Returns input md, not modified
pub struct WriteMd {
    input: Box<dyn Eval<(), MdDoc>>,
    output: PathBuf,
}
//
//
impl WriteMd {
    ///
    /// Returns [WriteMd] new instance
    pub fn new(path: &Path, input: impl Eval<(), MdDoc> +'static) -> Self {
        Self {
            input: Box::new(input),
            output: path.to_owned(),
        }
    }
}
//
//
impl Eval<(), MdDoc> for WriteMd {
    /// Writes input md into file with te specified path
    /// - Returns input md, not modified
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        let md_path = self.output.with_extension("md");
        let mut file = std::fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&md_path)
            .unwrap();
        file.write_all(doc.joined().as_bytes()).unwrap();
        doc
    }
}