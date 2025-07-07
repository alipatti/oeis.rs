mod api;
mod cli;

use clap::Parser;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::Display,
    io::{stdin, stdout},
};
use termion::{input::TermRead, raw::IntoRawMode};

use crate::{
    api::open_sequence,
    cli::{Args, Cli},
};

static OEIS_URL: &str = "https://oeis.org";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.sequence.is_empty() {
        return Err("Sequence cannot be empty.".into());
    }

    // parse command line args and load CLI
    let _stdout = stdout().into_raw_mode()?;
    let mut cli = Cli::from_args(&args)?;

    // handle non-interactive options
    if args.online {
        open::that(format!(
            "{OEIS_URL}/search?q={}",
            &args.sequence.join(",")
        ))?;
        return Ok(());
    }
    if args.lucky {
        open_sequence(cli.sequences[0].id);
        return Ok(());
    }

    print!("{}", termion::cursor::Hide);

    cli.print_cli();

    for key in stdin().keys() {
        match cli.process_keystroke(key?) {
            Some(_) => cli.print_cli(),
            None => break, // user exited program
        };
    }

    // show cursor again
    print!("{}", termion::cursor::Show);

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Sequence {
    /// The OEIS id of the sequence stored as an integer.
    /// For example, `A000040` (the primes) is stored as the integer `40`.
    #[serde(rename = "number")]
    pub(crate) id: usize,

    /// The name of the sequence.
    pub(crate) name: String,

    /// The actual values of the sequence (zero indexed).
    #[serde(
        deserialize_with = "crate::api::deserialize_sequence",
        rename = "data"
    )]
    pub(crate) values: Vec<i64>,
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{:0>6}: {}", self.id, self.name)
    }
}

impl Sequence {
    fn oeis_id(&self) -> String {
        format!("A{:0>6}", self.id)
    }
}
