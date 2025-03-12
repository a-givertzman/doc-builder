use std::{collections::HashMap, path::{Path, PathBuf}, time::Duration};
// use base64::{engine::general_purpose, Engine};
// use image::{DynamicImage, ImageFormat};

use regex::Regex;

use super::{
    doc_dir::DocDir, eval::Eval, html_embedd_svg::HtmlEmbeddSvg, html_fill_title_page::HtmlFillTitle, html_regex_replace::HtmlRegexReplace, html_replace_pagebreaks::HtmlReplacePageBreaks, html_to_pdf::{HtmlToPdf, HtmlToPdfOptions}, html_use_template::HtmlUseTemplate, md_doc::MdDoc, md_to_html::MdToHtml, write_html::WriteHtml, write_md::WriteMd
};
///
/// Converts multiple `markdown` documents into the single `Html`
/// 
/// Based on:
/// https://github.com/kivikakk/comrak
pub struct ComrakConvert {
    path: PathBuf,
    output: PathBuf,
    assets: PathBuf,
    template: PathBuf,
    // math_script: PathBuf,
}
//
//
impl ComrakConvert {
    ///
    /// Returns ComracConvert new instance
    /// - `path` - folder with markdown documents
    /// - `assets` - folder with asset files
    pub fn new(path: impl AsRef<Path>, output: impl AsRef<Path>, assets: impl AsRef<Path>, template: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            output: output.as_ref().to_path_buf(),
            assets: assets.as_ref().parent().unwrap_or(assets.as_ref()) .to_path_buf(),
            template: template.as_ref().to_path_buf(),
            // math_script: PathBuf::from("src/mathJax/es5/tex-mml-chtml.js"),
        }
    }
    // ///
    // /// 
    // fn image_to_base64(img: &DynamicImage) -> String {
    //     let mut image_data: Vec<u8> = Vec::new();
    //     img.write_to(&mut std::io::Cursor::new(&mut image_data), ImageFormat::Png)
    //         .unwrap();
    //     let res_base64 = general_purpose::STANDARD.encode(image_data);
    //     format!("data:image/png;base64,{}", res_base64)
    // }
    // ///
    // /// Embedding formula math module js script
    // fn embedd_math(html: &str, path: &Path) -> String {
    //     let script = fs::read_to_string(path).unwrap();
    //     let math_re = format!(r"[ \t]*//[ \t]*{}.*", Self::MATH_MODULE);
    //     log::debug!("embedd_math | math_module: {}", script.len());
    //     log::debug!("embedd_math | math_re: '{}'", math_re);
    //     let re = Regex::new(&math_re).unwrap();
    //     let html = re.replace(html, script).into_owned();
    //     // log::debug!("embedd_math | html: '{}'", html);
    //     html
    // }
    ///
    /// Performs a conversion
    pub fn convert(&self) {
        let (target_html, target_pdf) = if self.output.is_dir() {
            (
                self.output.join("doc.html"),
                self.output.join("doc.pdf"),
            )
        } else {
            (
                self.output.with_extension("html"),
                self.output.with_extension("pdf"),
            )
        };
        let _ = HtmlToPdf::new(
            &target_html,
            &target_pdf,
            HtmlToPdfOptions {
                landscape: false,
                scale: None,
                wait_before_print: Some(Duration::from_secs(1)),
            },
            WriteHtml::new(
                &target_html,
                HtmlRegexReplace::new(
                    Regex::new(r#"class="language-mermaid""#).unwrap(),
                    HashMap::from([(0, r#"class="mermaid""#)]),
                    HtmlReplacePageBreaks::new(
                        HtmlFillTitle::new(
                            HtmlUseTemplate::new(
                                &self.template,
                                HtmlEmbeddSvg::new(
                                    &self.assets,
                                    MdToHtml::new(
                                        WriteMd::new(
                                            &self.output,
                                            MdDoc::new(
                                                DocDir::new(&self.path).scan("md"),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )
        .eval(());
    }
}
