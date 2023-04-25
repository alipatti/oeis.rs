use std::{
    cmp::{max, min},
    io::{stdout, Write},
};

use clap::{command, Parser};
use colored::Colorize;
use itertools::Itertools;
use termion::{
    color::{self, Fg},
    cursor::{self, DetectCursorPos},
    event::Key,
    raw::IntoRawMode,
    style, terminal_size,
};
use textwrap::Options;

use crate::{
    api::{self, open_sequence},
    Sequence,
};

static N_TERMS: usize = 15; // number of terms of the sequence to show

#[derive(Parser)]
#[command(
    author,
    about,
    long_about = "A command line interface to the OEIS.\r\n\r\nUse the arrow keys to navigate the UI and the enter key to view the sequence on the OEIS. Press esc or q to exit."
)]
pub(crate) struct Args {
    pub(crate) sequence: Vec<String>,

    /// Immediately go to the OEIS page for the first result.
    #[arg(short, long)]
    pub(crate) lucky: bool,

    /// Search on the OEIS website.
    #[arg(short, long)]
    pub(crate) online: bool,
}

pub(crate) struct Cli {
    pub(crate) width: usize,
    pub(crate) selected_index: usize,
    pub(crate) sequences: Vec<Sequence>,
    origin: (u16, u16),
}

impl Cli {
    pub(crate) fn from_args(
        args: &Args,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let query: Vec<i32> = args
            .sequence
            .iter()
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;

        let sequences = api::search(&query)?;

        Ok(Cli {
            origin: stdout().into_raw_mode()?.cursor_pos()?,
            width: terminal_size()?.0 as usize,
            selected_index: 0,
            sequences,
        })
    }

    pub(crate) fn print_cli(&mut self) {
        // reset cursor
        print!("{}", cursor::Goto(self.origin.0, self.origin.1 + 1),);
        stdout().flush().unwrap();

        for (i, seq) in self.sequences.iter().enumerate() {
            let is_selected = i == self.selected_index;
            self.print_menu_item(seq, is_selected);
        }

        // print info for selected sequence
        println!("{}", termion::clear::AfterCursor);
        self.print_full_sequence_info();
    }

    pub(crate) fn print_menu_item(&self, seq: &Sequence, selected: bool) {
        let prefix = if selected {
            format!("> {}{}", style::Bold, Fg(color::Green))
        } else {
            "  ".to_string()
        };

        let seq_print_width = self.width - "> ...".len();
        let ellipsis = if seq.to_string().len() > seq_print_width {
            "..."
        } else {
            ""
        };

        print!(
            "{}{prefix}{:.seq_print_width$}{ellipsis}{}\r\n",
            termion::clear::CurrentLine,
            seq.to_string(),
            style::Reset
        );
    }

    pub(crate) fn process_keystroke(&mut self, key: Key) -> Option<()> {
        match key {
            // exit
            Key::Esc | Key::Ctrl('c') | Key::Char('q') => None,

            // go to OEIS page online
            Key::Char('\n') => {
                open_sequence(self.selected_sequence().id);
                None
            }

            // change selection
            // (strange incrementing to deal with usize not allowing negatives)
            Key::Up | Key::Char('k') => {
                self.selected_index = max(1, self.selected_index) - 1;
                Some(())
            }
            Key::Down | Key::Char('j') => {
                self.selected_index =
                    min(self.selected_index + 1, self.sequences.len() - 1);
                Some(())
            }

            // unknown input. sound bell
            _ => {
                print!("{}", 7 as char);
                Some(())
            }
        }
    }

    /// Prints information about the selected sequence.
    pub(crate) fn print_full_sequence_info(&self) {
        let seq = self.selected_sequence();

        print!("{}{}\r\n", "Sequence: ".cyan().bold(), seq.oeis_id());

        // print the full name of the sequence
        let header = "Description: ".cyan().bold().to_string();
        let wrap_options = Options::new(self.width).initial_indent(&header);
        print!(
            "{}\r\n",
            textwrap::wrap(&seq.name, wrap_options).join("\r\n"),
        );

        // print the first few sequence values
        print!(
            "{} {}...\r\n",
            "Sequence:".cyan().bold(),
            seq.values.iter().take(N_TERMS).join(", "),
        );

        stdout().flush().unwrap();
    }

    pub(crate) fn selected_sequence(&self) -> &Sequence {
        &self.sequences[self.selected_index]
    }
}
