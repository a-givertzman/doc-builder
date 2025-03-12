use mermaid_rs::Mermaid;
use regex::Regex;
use super::{eval::Eval, md_doc::MdDoc};

///
/// Returns html, with mermaid diagrams converted into svg
pub struct HtmlMermaidToSvg {
    input: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl HtmlMermaidToSvg {
    ///
    /// Returns [MermaidToSvg] new instance
    pub fn new(input: impl Eval<(), MdDoc> +'static) -> Self {
        Self {
            input: Box::new(input),
        }
    }
    ///
    /// 
    fn convert(md: &str, mermaid: Mermaid) -> String {
        let re = Regex::new(r#"```mermaid(?:\s)?([\S\s]+)```"#).unwrap();
        re.replace_all(md, |caps: &regex::Captures| {
            match caps.get(1) {
                Some(code) => {
                    let code = code.as_str();
                    log::warn!("HtmlMermaidToSvg.eval | Mermaid; {:#?}", code);
                    match mermaid.render(code) {
                        Ok(svg) => {
                            log::warn!("HtmlMermaidToSvg.eval | SVG; {:#?}", svg);
                            svg
                        },
                        Err(err) => {
                            log::warn!("HtmlMermaidToSvg.eval | Error; {:#?}", err);
                            code.to_owned()
                        },
                    }
                },
                None => {
                    log::warn!("HtmlMermaidToSvg.eval | Group 1 missed in capture; {:#?}", caps);
                    String::from("!!! MERMAID REMOVED !!!")
                },
            }
        }).into_owned()
        // let logo = re.captures(input).map_or("", |caps| caps.get(1).map_or("", |logo| logo.as_str())).to_owned();

    }
}
//
//
impl Eval<(), MdDoc> for HtmlMermaidToSvg {
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        match Mermaid::new() {
            Ok(mermaid) => {
                let md = Self::convert(&doc.markdown, mermaid);
                doc.with_md(md)
            }
            Err(err) => {
                log::warn!("HtmlMermaidToSvg.eval | Error; {:#?}", err);
                doc
            },
        }
    }
}