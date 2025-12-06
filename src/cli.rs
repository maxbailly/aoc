use clap::{Parser, Subcommand};

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

/* ---------- */

#[derive(Debug, Clone, Copy, Subcommand)]
pub(crate) enum Y2015 {
    All,
    Day1,
}

impl Y2015 {
    fn run(&self) {
        match self {
            Y2015::All => self.all(),
            Y2015::Day1 => crate::years::y2015::day1::resolve(),
        }
    }

    fn all(&self) {
        crate::years::y2015::resolve_all();
    }
}
