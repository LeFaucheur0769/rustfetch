//! This file handles all CLI flags such as -a or --all to display all options.
//! It also manages the --help text with custom comments.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rustfetch", version)]
pub struct Cli {
    #[arg(short, long, help = "Display all info regardless of config")]
    pub all: bool,
    #[arg(long, help = "Regenerates the .toml config file with standard values")]
    pub reset_config: bool,
    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "Adds padding on the logo, default is 5"
    )]
    // TODO: Handle incorrect use cases if the user inputs a value out of range
    pub padding: u8,
}
