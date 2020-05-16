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
  write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

  write!(stdout, "■").unwrap();
  write!(stdout, "{}", cursor::Goto(2, 2)).unwrap();
  write!(stdout, "□").unwrap();
  write!(stdout, "{}", cursor::Goto(3, 3)).unwrap();
  write!(stdout, "■").unwrap();
  write!(stdout, "{}", cursor::Goto(4, 4)).unwrap();
  write!(stdout, "□").unwrap();

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
