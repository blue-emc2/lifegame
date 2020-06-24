extern crate termion;

use std::io::Read;
use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, cursor};

use rand::Rng;
use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Cell {
  alive: bool,
}

struct Game {
  width: u16,
  height: u16,
  x: u16,
  y: u16,
  stdout: RawTerminal<Stdout>,
  grid: Box<[Cell]>,
}

impl Game {
  fn game_loop(&mut self) {
    let mut stdin = async_stdin().bytes();
    let mut rng = rand::thread_rng();

    // game loop
    loop {
      // cell: 細胞
      for (i, _cell) in (0_u16..).zip(self.grid.iter()) {
        self.x = (i % self.width) + 1;
        self.y = (i / self.height) + 1;

        let mut print = "";
        let number = rng.gen_range(0, 10);
        if (number % 2) == 0 {
          print = &"■";
        }

        write!(self.stdout, "{}", cursor::Goto(self.x, self.y)).unwrap();
        write!(self.stdout, "{}", print).unwrap();
      }

      self.stdout.flush().unwrap();

      let b = stdin.next();
      if let Some(Ok(b'q')) = b {
        return;
      }

      thread::sleep(Duration::from_millis(750));
    }
  }
}

fn game_init(mut stdout: RawTerminal<Stdout>) -> Game {
  let width: u16 = 16;
  let height: u16 = 16;
  let size = width * height;
  let vec = vec![Cell { alive: false }; size as usize].into_boxed_slice();
  let mut x: u16 = 0;
  let mut y: u16 = 0;

  write!(stdout, "{}", clear::All).unwrap();

  // game init
  // cell: 細胞
  for (i, _cell) in (0_u16..).zip(vec.iter()) {
    x = (i % width) + 1;
    y = (i / height) + 1;

    write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
    write!(stdout, "{}", "□").unwrap();
  }

  stdout.flush().unwrap();

  return Game {
    width: width,
    height: height,
    x: x,
    y: y,
    stdout: stdout,
    grid: vec,
  };
}

fn main() {
  // Get and lock the stdios.
  let stdout = stdout();
  // We go to raw mode to make the control over the terminal more fine-grained.
  let stdout = stdout.into_raw_mode().unwrap();

  let mut game = game_init(stdout);

  game.game_loop();
}
