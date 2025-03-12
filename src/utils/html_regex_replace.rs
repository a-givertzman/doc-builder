use std::collections::HashMap;

use regex::Regex;
use super::{eval::Eval, md_doc::MdDoc};

///
/// Returns html, with replaced regex pattern
pub struct HtmlRegexReplace {
    re: Regex,
    to: HashMap<usize, String>,
    input: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl HtmlRegexReplace {
    ///
    /// Returns [HtmlRegexReplace] new instance
    /// - `to` - map of matched group indexes and associated strings to be replaced
    ///     - 0 - replace whole matched
    ///     - 1 - replace group 1 of matched
    ///     - 2 - replace group 2 of matched...etc...
    pub fn new(re: Regex, to: HashMap<usize, &str>, input: impl Eval<(), MdDoc> +'static) -> Self {
        Self {
            re,
            to: to.into_iter().map(|(key, value)| (key, value.to_owned())).collect(),
            input: Box::new(input),
        }
    }
    ///
    /// Replaces all matched in the `html`
    /// - `re` - regex to be matched
    /// - `html` - original string
    /// - `to` - collection of strings indexed by related group number
    fn replace(re: &Regex, html: &str, to: &HashMap<usize, String>) -> String {
        let mut new = String::new();
        let mut last_match = 0;
        for caps in re.captures_iter(html) {
            for (i, to) in to {
                if let Some(matched) = caps.get(*i) {
                    log::debug!("HtmlRegexReplace.replace | found; {:#?}", matched.as_str());
                    new.push_str(&html[last_match..matched.start()]);
                    new.push_str(to);
                    last_match = matched.end();
                }
            }
        }
        new.push_str(&html[last_match..]);
        new
    }
}
//
//
impl Eval<(), MdDoc> for HtmlRegexReplace {
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        let html = Self::replace(&self.re, &doc.html, &self.to);
        doc.with_html(html)
    }
}