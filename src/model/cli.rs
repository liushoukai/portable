use clap::Parser;

/// A tool to generate git commit messages using an AI model.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Automatically execute the generated git commit command.
    #[arg(short, long, default_value_t = false)]
    pub auto: bool,
}
