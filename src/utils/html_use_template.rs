use std::path::{Path, PathBuf};
use super::{eval::Eval, md_doc::MdDoc};

///
/// Returns html, with specified html template, where body contains input html body 
/// - `MdDoc::BODY_CONTENT` => html_bode
pub struct HtmlUseTemplate {
    template: PathBuf,
    input: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl HtmlUseTemplate {
    ///
    /// Returns [HtmlUseTemplate] new instance
    pub fn new(template: &Path, input: impl Eval<(), MdDoc> +'static) -> Self {
        Self {
            template: template.to_owned(),
            input: Box::new(input),
        }
    }

}
//
//
impl Eval<(), MdDoc> for HtmlUseTemplate {
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        let html = match std::fs::read_to_string(&self.template) {
            Ok(template) => {
                template.replace(MdDoc::BODY_CONTENT, &doc.html)
            }
            Err(_) => {
                log::warn!("convert | Template is not found in: {}", self.template.display());
                return doc;
            }
        };
        doc.with_html(html)
    }
}