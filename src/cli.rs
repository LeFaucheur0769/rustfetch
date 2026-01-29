use clap::Parser;

/// CLI options for rustfetch
#[derive(Parser, Debug)]
#[command(name = "rustfetch", version)]
pub struct Cli {
    #[arg(short, long)]
    pub all: bool,
}
