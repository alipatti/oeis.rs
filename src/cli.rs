use std::{
    cmp::{max, min},
    io::{stdout, Stdout, Write},
};

use colored::Colorize;
use itertools::Itertools;
use termion::{
    color::{self, Fg},
    cursor::{self, DetectCursorPos},
    event::Key,
    raw::{IntoRawMode, RawTerminal},
    style, terminal_size,
};

use crate::{api, Args, Sequence, OEIS_URL};

pub(crate) struct Cli {
    pub(crate) width: usize,
    pub(crate) cursor_home: (u16, u16),
    pub(crate) stdout: RawTerminal<Stdout>,
    pub(crate) selected_index: usize,
    pub(crate) sequences: Vec<Sequence>,
}

impl Cli {
    pub(crate) fn from_args(
        args: Args,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let query = args.sequence.join(",");
        let mut stdout = stdout().into_raw_mode()?;
        let cursor_home = stdout.cursor_pos()?;

        // prepare stdout/stdin

        Ok(Cli {
            stdout,
            cursor_home,
            width: terminal_size()?.0 as usize,
            selected_index: 0,
            sequences: api::search(&query)?,
        })
    }

    pub(crate) fn print_cli(&mut self) {
        // reset cursor
        print!("{}", cursor::Goto(self.cursor_home.0, self.cursor_home.1));
        self.stdout.flush().unwrap();

        // print sequences and abbreviated titles
        for (i, seq) in self.sequences.iter().enumerate() {
            let is_selected = i == self.selected_index;
            self.print_menu_item(seq, is_selected);
        }

        self.print_selected_sequence_info();
        self.stdout.flush().unwrap();
    }

    pub(crate) fn print_menu_item(&self, seq: &Sequence, selected: bool) {
        let prefix = if selected {
            format!(" > {}{}", style::Bold, Fg(color::Green))
        } else {
            "   ".to_string()
        };

        let oeis_id = format!("A{:0>6}", seq.id);
        let title = {
            let description_width = self.width - 20;

            let ellipsis = if seq.name.len() > description_width {
                "..."
            } else {
                ""
            };
            format!("{:.description_width$}{ellipsis}", seq.name,)
        };

        print!(
            "{}{prefix}{oeis_id}: {title}{}\r\n",
            termion::clear::CurrentLine,
            style::Reset
        );
    }

    pub(crate) fn process_keystroke(
        &mut self,
        key: Key,
    ) -> Result<Option<()>, Box<dyn std::error::Error>> {
        Ok(match key {
            // exit
            Key::Esc | Key::Ctrl('c') | Key::Char('q') => None,

            // go to OEIS page online
            Key::Char('\n') => {
                self.selected_sequence();
                let url = format!(
                    "{}/A{:0>6}",
                    OEIS_URL,
                    self.selected_sequence().id
                );
                open::that(url)?;
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
                write!(self.stdout, "{}", 7 as char)?;
                Some(())
            }
        })
    }

    pub(crate) fn print_selected_sequence_info(&self) {
        // print information about the selected sequence
        let selected_sequence = &self.sequences[self.selected_index];
        print!(
            "\r\n{:-^width$}\r\n",
            format!(" A{:0>6} ", selected_sequence.id).bold().cyan(),
            width = self.width,
        ); // header

        print!(
            "{}{}...",
            termion::clear::CurrentLine,
            &selected_sequence.values[..15]
                .iter()
                .map(|x| x.to_string())
                .join(", ")
        );
    }

    pub(crate) fn selected_sequence(&self) -> &Sequence {
        &self.sequences[self.selected_index]
    }
}
