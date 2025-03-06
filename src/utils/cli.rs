use clap::Parser;
/// 
/// Converts markdown files to the solid html document
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// 
    /// Path to operate on
    pub name: String,
    /// 
    /// Target path
    /// -o, --output
    #[arg(short, long)]
    pub output: Option<String>,
    /// 
    /// Assets path
    /// --assets
    #[arg(long)]
    pub assets: Option<String>,
    ///
    /// Template
    /// --template
    #[arg(long)]
    pub template: Option<String>,
}
