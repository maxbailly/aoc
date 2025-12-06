mod cli;
mod years;

use clap::Parser;

use crate::cli::Cli;

/* ---------- */

fn main() {
    Cli::parse().run();
}
