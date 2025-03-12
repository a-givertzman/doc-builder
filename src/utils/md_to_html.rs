use std::sync::Arc;

use super::{eval::Eval, md_doc::MdDoc};

///
/// Returns a `html` representation of the markdown `document`
pub struct MdToHtml {
    input: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl MdToHtml {
    ///
    /// Returns [MdToHtml] new instance
    pub fn new(input: impl Eval<(), MdDoc> +'static) -> Self {
        Self { input: Box::new(input) }
    }
    ///
    /// Returns a `html` representation of the markdown `document`
    fn parse(document: &str) -> String {
        // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
        let arena = comrak::Arena::new();
        // Parse the document into a root `AstNode`
        let root = comrak::parse_document(
            &arena,
            document,
            &comrak::Options {
                extension: comrak::ExtensionOptions::builder()
                    .strikethrough(true)
                    .tagfilter(true)
                    .table(true)
                    .autolink(true)
                    .tasklist(true)
                    .superscript(true)
                    // .header_ids(true)
                    .footnotes(true)
                    .description_lists(true)
                    // .front_matter_delimiter(true)
                    .multiline_block_quotes(true)
                    // .math_dollars(true)
                    // .math_code(true)
                    .wikilinks_title_after_pipe(true)
                    .wikilinks_title_before_pipe(true)
                    .underline(true)
                    .subscript(true)
                    .spoiler(true)
                    .greentext(true)
                    .image_url_rewriter(Arc::new(|url: &str| {
                        log::debug!("url: {}", url);
                        // format!("https://safe.example.com?url={}", url)
                        url.to_owned()
                    }))
                    // .link_url_rewriter(true)
                    .build(),
                parse: comrak::ParseOptions::builder()
                    .smart(true)
                    // .default_info_string()
                    // .relaxed_tasklist_matching()
                    // .relaxed_autolinks()
                    // .broken_link_callback()
                    .build(),
                render: comrak::RenderOptions::builder()
                    // .hardbreaks()
                    // .github_pre_lang()
                    // .full_info_string()
                    // .width()
                    .unsafe_(true)
                    // .escape()
                    // .list_style()
                    // .sourcepos()
                    // .experimental_inline_sourcepos()
                    // .escaped_char_spans()
                    // .ignore_setext()
                    // .ignore_empty_links()
                    // .gfm_quirks()
                    // .prefer_fenced()
                    // .figure_with_caption()
                    // .tasklist_classes()
                    // .ol_width()
                    .build()
            },
        );
        // Iterate over all the descendants of root.
        // for node in root.descendants() {
        //     if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
        //         // If the node is a text node, replace `orig_string` with `replacement`.
        //         *text = text.replace(orig_string, replacement)
        //     }
        // }
        let mut html = vec![];
        comrak::format_html(root, &comrak::Options::default(), &mut html).unwrap();
        String::from_utf8(html).unwrap()
    }
}
//
//
impl Eval<(), MdDoc> for MdToHtml {
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        let html = Self::parse(&doc.markdown);
        doc.with_html(html)
    }
}