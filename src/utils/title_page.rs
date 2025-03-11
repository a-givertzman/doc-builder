use std::{fs, path::Path};

use regex::Regex;

///
/// Find and reterns title page contents as:
/// 
/// `(logo, addr, name, descr)`
/// 
/// Returns empty string if field is not found
#[derive(Clone)]
pub struct Title {
    pub raw: String,
    pub logo: String,
    pub addr: String,
    pub name: String, 
    pub descr: String
}
impl Title {
    pub const LOGO: &str = "======================title-section-logo======================";
    pub const ADDR: &str = "======================title-section-addr======================";
    pub const NAME: &str = "======================title-section-name======================";
    pub const DESCR: &str = "======================title-section-descr======================";
    // ///
    // /// Returns [Title] new empty instance
    // pub fn new() -> Self {
    //     Self {
    //         raw: String::new(),
    //         logo: String::new(),
    //         addr: String::new(),
    //         name: String::new(),
    //         descr: String::new(),
    //     }
    // }
    /// Find and reterns [Title] page contains of fields:
    /// - logo,
    /// - addr,
    /// - name,
    /// - descr
    /// 
    /// Returns empty strings instead missed fields
    /// 
    /// - `input` - string, contains markdown
    pub fn from(path: &Path) -> Option<Self> {
        if path.is_file() {
            match fs::read_to_string(path) {
                Ok(input) => {
                    let (logo, addr, name, descr) = Self::find_title(&input);
                    if format!("{}{}{}{}", logo, addr, name, descr).is_empty() {
                        // log::warn!("Title.from | Title not found in: {:#?}", input);
                        None
                    } else {
                        // log::warn!("Title.from | Title: {:#?}", input);
                        Some(Self {raw: input, logo, addr, name, descr})
                    }
                }
                Err(err) => {
                    log::warn!("Title.from | Error: {:#?}", err);
                    None
                },
            }
        } else {
            None
        }
    }
    ///
    /// Find and reterns title page as:
    /// 
    /// `(logo, addr, name, descr)`
    /// 
    /// Returns `input` with removed foun blocks
    /// Returns empty string if field is not found
    fn find_title(input: &str) -> (String, String, String, String) {
        let re = Regex::new(r#"=+title-section-logo=+(?:\r\n|\r|\n)+(\S+)(?:\r\n|\r|\n)+=+title-section-logo=+"#).unwrap();
        let logo = re.captures(input).map_or("", |caps| caps.get(1).map_or("", |logo| logo.as_str())).to_owned();
        let re = Regex::new(r#"=+title-section-addr=+(?:\r\n|\r|\n)([\S\s]+)(?:\r\n|\r|\n)=+title-section-addr=+"#).unwrap();
        let addr = re.captures(input).map_or("", |caps| caps.get(1).map_or("", |logo| logo.as_str())).to_owned();
        let re = Regex::new(r#"=+title-section-name=+(?:\r\n|\r|\n)([\S\s]+)(?:\r\n|\r|\n)=+title-section-name=+"#).unwrap();
        let name = re.captures(input).map_or("", |caps| caps.get(1).map_or("", |logo| logo.as_str())).to_owned();
        let re = Regex::new(r#"=+title-section-descr=+(?:\r\n|\r|\n)([\S\s]+)(?:\r\n|\r|\n)=+title-section-descr=+"#).unwrap();
        let descr = re.captures(input).map_or("", |caps| caps.get(1).map_or("", |logo| logo.as_str())).to_owned();
        (logo, addr, name, descr)
    }
}
//
//
impl std::fmt::Debug for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Title")
            // .field("raw", &self.raw)
            .field("logo", &self.logo)
            .field("addr", &self.addr)
            .field("name", &self.name)
            .field("descr", &self.descr)
            .finish()
    }
}
