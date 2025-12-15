use clap::Parser;

use crate::years::Y2015;

/* ---------- */

#[derive(Debug, Clone, Copy, Parser)]
pub(crate) enum Cli {
    All,
    #[command(subcommand)]
    Y2015(Y2015),
}

impl Cli {
    pub(crate) fn run(&self) {
        match &self {
            Self::All => self.all(),
            Self::Y2015(year) => year.run(),
        }
    }

    fn all(&self) {
        crate::years::y2015::resolve_all();
    }
}
