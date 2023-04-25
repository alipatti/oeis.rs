mod api;
mod cli;

use clap::{command, Parser};
use serde::Deserialize;
use std::{collections::HashMap, io::stdin};
use termion::input::TermRead;

static OEIS_URL: &str = "https://oeis.org";

#[derive(Parser)]
#[command(
    author = "Alistair Pattison",
    about = "A command line interface to the OEIS."
)]
struct Args {
    sequence: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line arguments
    let args = Args::parse();

    if args.sequence.is_empty() {
        println!("Sequence cannot be empty.");
        return Ok(());
    }

    // hide cursor
    print!("{}", termion::cursor::Hide);

    let mut cli = cli::Cli::from_args(args)?;

    cli.print_cli();

    for key in stdin().keys() {
        match cli.process_keystroke(key?)? {
            Some(_) => cli.print_cli(),
            None => break,
        };
    }

    // show cursor again
    print!("{}", termion::cursor::Show);

    Ok(())
}

#[derive(Deserialize)]
pub struct Sequence {
    /// The OEIS id of the sequence stored as an integer.
    /// For example, `A000040` (the primes) is stored as the integer `40`.
    #[serde(rename = "number")]
    pub(crate) id: i32,

    /// The name of the sequence.
    pub(crate) name: String,

    /// The actual values of the sequence (zero indexed).
    #[serde(
        deserialize_with = "crate::api::deserialize_sequence",
        rename = "data"
    )]
    pub(crate) values: Vec<i64>,

    #[serde(rename = "comment")]
    pub(crate) _comments: Option<Vec<String>>,

    #[serde(rename = "reference")]
    pub(crate) _references: Option<Vec<String>>,

    #[serde(flatten)]
    pub(crate) _extra: HashMap<String, serde_json::Value>,
}
