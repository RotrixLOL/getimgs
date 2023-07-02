use clap::Parser;

/// Rip Grep. Simple Grep clone
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File with all the links of the images (one link per line)
    pub file: String,

    /// Directory to save all the images
    #[arg(short, long, default_value_t = String::from("images"))]
    pub output: String,

    #[arg(short, long, default_value_t = 50)]
    pub threads: u32,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

