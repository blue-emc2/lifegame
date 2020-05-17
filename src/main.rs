extern crate termion;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use std::io::{stdin, stdout, Write};

fn main() {
  // Get and lock the stdios.
  let stdout = stdout();
  // We go to raw mode to make the control over the terminal more fine-grained.
  let mut stdout = stdout.into_raw_mode().unwrap();

  write!(stdout, "{}", clear::All).unwrap();

  let width = 8;
  let height = 8;

  for h in 1..=height {
    for w in 1..=width {
      write!(stdout, "{}", cursor::Goto(w, h)).unwrap();
      write!(stdout, "■").unwrap();
    }
  }

  stdout.flush().unwrap();

  let stdin = stdin();

  for evt in stdin.events() {
    let e = evt.unwrap();
    match e {
      Event::Key(Key::Char('q')) => break,
      _ => {}
    }
  }
}
