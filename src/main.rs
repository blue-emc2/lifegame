extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use std::io::{stdin, stdout, Write};

use rand::Rng;

fn main() {
  // Get and lock the stdios.
  let stdout = stdout();
  // We go to raw mode to make the control over the terminal more fine-grained.
  let mut stdout = stdout.into_raw_mode().unwrap();

  write!(stdout, "{}", clear::All).unwrap();

  let width: u16 = 16;
  let height: u16 = 16;
  let size = width * height;
  let vec = vec!["□"; size as usize];
  let mut x: u16;
  let mut y: u16;

  let mut stdin = stdin().keys();

  // game init
  // cell: 細胞
  for (i, cell) in (0_u16..).zip(vec.iter()) {
    x = (i % width) + 1;
    y = (i / height) + 1;

    write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
    write!(stdout, "{}", cell).unwrap();
  }

  stdout.flush().unwrap();

  let mut rng = rand::thread_rng();

  // game loop
  loop {
    // cell: 細胞
    for (i, cell) in (0_u16..).zip(vec.iter()) {
      x = (i % width) + 1;
      y = (i / height) + 1;

      let mut print = cell;
      let number = rng.gen_range(0, 10);
      if (number % 2) == 0 {
        print = &"■";
      }

      write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
      write!(stdout, "{}", print).unwrap();
    }

    stdout.flush().unwrap();

    let key = stdin.next().unwrap().unwrap();
    if key == Key::Char('q') {
      return;
    }
  }
}
