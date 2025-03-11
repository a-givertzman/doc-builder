use super::{eval::Eval, md_doc::MdDoc};

///
/// Returns html, with replaced `MdDoc::PAGEBREAK` => `<div class="pagebreak"> </div>`
pub struct HtmlReplacePageBreaks {
    input: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl HtmlReplacePageBreaks {
    ///
    /// Returns [HtmlReplacePageBreaks] new instance
    pub fn new(input: impl Eval<(), MdDoc> +'static) -> Self {
        Self { input: Box::new(input) }
    }
}
//
//
impl Eval<(), MdDoc> for HtmlReplacePageBreaks {
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        let html = doc.html.replace(MdDoc::PAGEBREAK, "<div class=\"pagebreak\"> </div>");
        doc.with_html(html)
    }
}