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

  let width: u16 = 16;
  let height: u16 = 16;
  let size = width * height;
  let vec = vec!["■ "; size as usize];
  let mut x: u16;
  let mut y: u16;

  // cell: 細胞
  for (i, cell) in (0_u16..).zip(vec.iter()) {
    x = (i % width) + 1;
    y = (i / height) + 1;

    let mut print = cell;
    if (x % 2) == 0 {
      print = &"□";
    }

    write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
    write!(stdout, "{}", print).unwrap();
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
