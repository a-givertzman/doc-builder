use std::{io::{BufReader, Read}, path::{Path, PathBuf}};

use regex::Regex;

use super::{eval::Eval, md_doc::MdDoc};

///
/// Returns html, with replaced `MdDoc::PAGEBREAK` => `<div class="pagebreak"> </div>`
pub struct HtmlEmbeddSvg {
    assets: PathBuf,
    input: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl HtmlEmbeddSvg {
    ///
    /// Returns [HtmlEmbeddSvg] new instance
    pub fn new(assets: &Path, input: impl Eval<(), MdDoc> +'static) -> Self {
        Self {
            assets: assets.to_owned(),
            input: Box::new(input),
        }
    }
    ///
    /// Embedding images into Html
    fn embedd_images(html: &str, assets: &Path) -> String {
        let mut result = String::new();
        let re = Regex::new(r#"(<img\s+?src=")(.*?)(".*?/>)"#).unwrap();
        let mut las_match = 0;
        for item in re.captures_iter(html) {
            // log::debug!("embedd_images | img: {:?}", item.get(2));
            if let (Some(prefix), Some(path), Some(sufix)) = (item.get(1), item.get(2), item.get(3)) {
                result.push_str(
                    &html[las_match..prefix.start()]
                );
                // let path_str = path.as_str();
                let path = if path.as_str().starts_with("/") {
                    assets.join(path.as_str().trim_start_matches("/"))
                } else {
                    assets.join(path.as_str())
                };
                if let Some(ext) = path.extension().and_then(std::ffi::OsStr::to_str) {
                    match ext {
                        "svg" => {
                            log::debug!("embedd_images | SVG img: {:?}", path);
                            match std::fs::File::open(&path) {
                                Ok(file) => {
                                    let mut img = String::new();
                                    match BufReader::new(file).read_to_string(&mut img) {
                                        Ok(_) => {
                                            result.push_str(&img);
                                        }
                                        Err(err) => {
                                            log::warn!("embedd_images | Error read img file: '{:?}': \n\t{:?}", path, err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    log::warn!("embedd_images | Error acces img file: '{:?}': \n\t{:?}", path, err);
                                }
                            }
                        }
                        _ => {
                            log::debug!("embedd_images | img by ref: {:?}...", path);
                            //     let img = image::ImageReader::open(&path).unwrap().decode().unwrap();
                            //     log::debug!("embedd_images | reading img: {:?} - Ok", path);
                            //     let img = Self::image_to_base64(&img);
                            //     let img = format!("{}{}{}", prefix.as_str(), img, sufix.as_str());
                            //     // <img src="">
                            result.push_str(prefix.as_str());
                            result.push_str(path.as_os_str().to_str().unwrap());
                            result.push_str(sufix.as_str());
                        }
                    }
                }
                las_match = sufix.end();
            }
        }
        result.push_str(
            &html[las_match..]
        );
        result
        // html.to_owned()
    }
}
//
//
impl Eval<(), MdDoc> for HtmlEmbeddSvg {
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.input.eval(());
        let html = Self::embedd_images(&doc.html, &self.assets);
        doc.with_html(html)
    }
}