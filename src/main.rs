mod utils;

use std::path::PathBuf;
use clap::Parser;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use utils::{cli::Cli, comrak_convert::ComrakConvert};

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    match Cli::try_parse() {
        Ok(cli) => {
            let path = PathBuf::from(cli.name);
            let assets = match cli.assets {
                Some(assets) => PathBuf::from(assets),
                None => PathBuf::from("./"),
            };
            let output = match cli.output {
                Some(output) => PathBuf::from(output),
                None => path.clone(),
            };
            let template = match cli.template {
                Some(template) => PathBuf::from(template),
                None => PathBuf::from("template.html"),
            };
            ComrakConvert::new(&path, &output, assets, template).convert();
        }
        Err(err) => {
            log::error!("{:#?}", err);
        }
    };
}
