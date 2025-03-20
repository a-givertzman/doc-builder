use std::{io::Write, path::{Path, PathBuf}, time::Duration};
use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptions};
use html2pdf::PaperSize;
use super::{eval::Eval, md_doc::MdDoc};

///
/// Conwerts `html` into PDF
/// - Returns original `html`
pub struct HtmlToPdf {
    dbg: String,
    input: PathBuf,
    output: PathBuf,
    options: HtmlToPdfOptions,
    doc: Box<dyn Eval<(), MdDoc>>,
}
//
//
impl HtmlToPdf {
    ///
    /// Returns [HtmlToPdf] new instance
    /// -o, --output <output>  Output file. By default, just change the input extension to PDF
    pub fn new(
        input: &Path,
        output: &Path,
        options: HtmlToPdfOptions,
        doc: impl Eval<(), MdDoc> +'static,
    ) -> Self {
        Self {
            dbg: "HtmlToPdf".to_owned(),
            input: input.to_owned(),
            output: output.to_owned(),
            options,
            doc: Box::new(doc)
        }
    }
    ///
    /// 
    fn print(input: &Path, output: &Path, options: &HtmlToPdfOptions) -> Result<(), Box<dyn std::error::Error>> {
        let lunch_options = LaunchOptions::default_builder()
            .ignore_certificate_errors(true)
            .window_size(Some((1024, 1024)))
            .sandbox(true)
            .build()?;
        let input = input.canonicalize()?;
        let input = input.as_os_str().to_str().ok_or(format!("Invalid input path: {:?}", input.to_str()))?;
        let input = format!("file://{input}");
        let browser = Browser::new(lunch_options)?;
        let tab = browser.new_tab()?;
        let tab = tab.navigate_to(&input)?.wait_until_navigated()?;
        if let Some(wait) = options.wait_before_print {
            log::info!("Waiting {:?} before export to PDF", wait);
            std::thread::sleep(wait);
        }
        let bytes = tab.print_to_pdf(Some(PrintToPdfOptions {
            landscape: Some(options.landscape),
            display_header_footer: Some(false),
            // print_background: Some(false),
            scale: options.scale,
            paper_width: Some(PaperSize::A4.paper_width()),
            paper_height: Some(PaperSize::A4.paper_height()),
            margin_top: Some(0.0),
            margin_bottom: Some(0.0),
            margin_left: Some(0.0),
            margin_right: Some(0.0),
            ..Default::default()
            // page_ranges: Default::default(),
            // ignore_invalid_page_ranges: Default::default(),
            // header_template: Default::default(),
            // footer_template: Default::default(),
            // prefer_css_page_size: Default::default(),
            // transfer_mode: Default::default(),
            // generate_document_outline: Default::default(),
            // generate_tagged_pdf: Default::default(),
        }))?;
        let mut file = std::fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(output)
            .unwrap();
        file.write_all(&bytes)?;
        Ok(())
    }
}
//
//
impl Eval<(), MdDoc> for HtmlToPdf {
    fn eval(&mut self, _: ()) -> MdDoc {
        let doc = self.doc.eval(());
        if let Err(err) = Self::print(&self.input, &self.output, &self.options) {
            log::warn!("{}.eval | Error: {:#?}", self.dbg, err);
        }
        doc
    }
}

///
/// OPTIONS:
/// -footer -  HTML template for the print footer. Should use the same format as the headerTemplate
/// -header -  HTML template for the print header. Should be valid HTML markup with following classes used
///                    to inject printing values into them: date for formatted print date, title for document
///                    title, url for document location, pageNumber for current page number, totalPages for total
///                    pages in the document. For example, `<span class=title></span>` would generate span
///                    containing the title
/// -margin -  Margin in inches You can define margin like this: '0.4' the value is applied for all side,
///                           '0.4 0.4' : first value is applied for top and bottom, second for left and right, '0.4 0.4
///                           0.4 0.4' : first value is applied for top then, right, then bottom, and last for left
/// --paper <paper>    Paper size. Supported values: A4, Letter, A3, Tabloid, A2, A1, A0, A5, A6
/// --range <range>    Paper ranges to print, e.g. '1-5, 8, 11-13'
/// --scale <scale>    Scale, default to 1.0
/// --wait <wait>      Time to wait in ms before printing. Examples: 150ms, 10s
#[derive(Debug, Clone)]
pub struct HtmlToPdfOptions {
    pub landscape: bool,
    pub scale: Option<f64>,
    pub wait_before_print: Option<Duration>,
}