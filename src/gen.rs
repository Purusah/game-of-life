use std::io::{stdin, stdout, Write};

use crate::space::{Space, State};
use rand::random;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

#[derive(PartialEq)]
pub enum Nature {
    Default = 0,
    Random = 1,
    Custom = 2,
}

struct Cursor {
    pub x: usize,
    pub y: usize,
}

struct Input {
    cursor: Cursor,
    size: Cursor,
    state: Space,
}

impl Input {
    fn init() -> Self {
        let height = 16;
        let width = 16;
        let mut space: Space = vec![vec![State::Dead; width]; height];
        for r in 0..height {
            for c in 0..width {
                space[r][c] = State::Dead;
            }
        }

        Self {
            cursor: Cursor { x: 1, y: 1 },
            size: Cursor {
                x: width,
                y: height,
            },
            state: space,
        }
    }

    fn inc_x(&mut self) {
        if self.cursor.x < 2 * self.size.x - 1 {
            self.cursor.x += 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cursor.x as u16, self.cursor.y as u16)
        );
    }

    fn dec_x(&mut self) {
        if self.cursor.x > 1 {
            self.cursor.x -= 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cursor.x as u16, self.cursor.y as u16)
        );
    }

    fn inc_y(&mut self) {
        if self.cursor.y < self.size.y {
            self.cursor.y += 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cursor.x as u16, self.cursor.y as u16)
        );
    }

    fn dec_y(&mut self) {
        if self.cursor.y > 1 {
            self.cursor.y -= 1;
        }
        println!(
            "{}",
            termion::cursor::Goto(self.cursor.x as u16, self.cursor.y as u16)
        );
    }

    fn invert(&mut self) {
        if self.cursor.x % 2 == 1 {
            return;
        }

        let cell = self.state[self.cursor.y - 1][self.cursor.x / 2 - 1];

        self.state[self.cursor.y - 1][self.cursor.x / 2 - 1] = if cell == State::Dead {
            State::Alive
        } else {
            State::Dead
        }
    }

    fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('c') => {
                    break;
                }
                Key::Left => {
                    self.dec_x();
                    self.show_document();
                }
                Key::Right => {
                    self.inc_x();
                    self.show_document();
                }
                Key::Up => {
                    self.dec_y();
                    self.show_document();
                }
                Key::Down => {
                    self.inc_y();
                    self.show_document();
                }
                Key::Char('\n') => {
                    self.invert();
                    self.show_document();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }

    fn set_cursor(&mut self, x: usize, y: usize) {
        self.cursor.x = x;
        self.cursor.y = y;
        println!(
            "{}",
            termion::cursor::Goto(self.cursor.x as u16, self.cursor.y as u16)
        );
    }

    fn show_document(&mut self) {
        let pos = &self.cursor;
        let (old_x, old_y) = (pos.x, pos.y);
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!(
            "{}Enter to input; Ctrl+C to continue{}\r",
            color::Fg(color::White),
            style::Reset
        );
        for r in 0..self.size.y {
            for c in 0..self.size.x {
                if self.state[r][c] == State::Dead {
                    print!("| ");
                } else {
                    print!("|▇");
                }
            }
            println!("\r");
        }

        self.set_cursor(old_x, old_y);
    }
}

pub fn gen_space(nature: Nature) -> Space {
    if nature == Nature::Default {
        return vec![
            vec![State::Dead, State::Alive, State::Dead, State::Dead],
            vec![State::Dead, State::Dead, State::Alive, State::Dead],
            vec![State::Alive, State::Alive, State::Alive, State::Dead],
            vec![State::Dead, State::Dead, State::Dead, State::Dead],
            vec![State::Dead, State::Dead, State::Dead, State::Dead],
            vec![State::Dead, State::Dead, State::Dead, State::Dead],
        ];
    }

    if nature == Nature::Random {
        let height = 16;
        let width = 16;
        let mut space: Space = vec![vec![State::Dead; width]; height];
        for r in 0..height {
            for c in 0..width {
                space[r][c] = if random() { State::Alive } else { State::Dead };
            }
        }
        return space;
    }

    let mut input = Input::init();
    input.show_document();
    input.run();

    return input.state;
}
