//! This file handles all CLI flags such as -a or --all to display all options.
//! It also manages the --help text with custom comments.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rustfetch", version)]
pub struct Cli {
    #[arg(short, long, help = "Display all info regardless of config")]
    pub all: bool,
}
