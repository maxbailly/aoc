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
    Day2,
    Day3,
    Day4,
    Day5,
}

impl Y2015 {
    fn run(&self) {
        match self {
            Self::All => self.all(),
            Self::Day1 => crate::years::y2015::day01::resolve(),
            Self::Day2 => crate::years::y2015::day02::resolve(),
            Self::Day3 => crate::years::y2015::day03::resolve(),
            Self::Day4 => crate::years::y2015::day04::resolve(),
            Self::Day5 => crate::years::y2015::day05::resolve(),
        }
    }

    fn all(&self) {
        crate::years::y2015::resolve_all();
    }
}
