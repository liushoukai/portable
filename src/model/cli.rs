use clap::Parser;

/// A tool to generate git commit messages using an AI model.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Automatically execute the generated git commit command.
    #[arg(short = 'a', long, default_value_t = false)]
    pub auto: bool,
    /// Timeout for AI API requests (in milliseconds)
    #[arg(short = 't', long, default_value_t = 30000)]
    pub timeout: u64,
}
