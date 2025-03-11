use crate::utils::title_page::Title;

use super::{eval::Eval, md_doc::MdDoc};

///
/// Returns html, with replaced `MdDoc::PAGEBREAK` => `<div class="pagebreak"> </div>`
pub struct HtmlFillTitle {
    input: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl HtmlFillTitle {
    ///
    /// Returns [HtmlFillTitle] new instance
    pub fn new(input: impl Eval<(), MdDoc> +'static) -> Self {
        Self {
            input: Box::new(input)
        }
    }
}
//
//
impl Eval<(), MdDoc> for HtmlFillTitle {
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        let html = &doc.html;
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
                return doc;
            }
        };
        doc.with_html(html)
    }
}