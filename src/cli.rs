use termion::{color, style};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};
use std::io::{Write, stdout, stdin};


#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub full: String,
}
pub struct Cli {
    items: Vec<Item>,
    current_item: usize,
    openned_item: Option<Item>,
    old_buf_lines: u16,
}
impl Cli {
    pub fn new() -> Cli {
        Cli {
            items: Vec::new(),
            current_item: 0,
            openned_item: None,
            old_buf_lines: 0,
        }
    }
    pub fn print(&mut self, print: &str) {
        let print = format!("{}\r\n", print);
        self.old_buf_lines = line_count(&print);
        print!("{}", print);
    }
    pub fn print_items(&mut self, items: Vec<Item>) {
        self.items = items;
        self.redraw();
        self.listen_keys();
    }
    fn clear(&self) {
        print!("{}{}", cursor::Up(self.old_buf_lines), clear::AfterCursor);
    }

    fn redraw(&mut self) {
        self.clear();
        match self.openned_item.clone() {
            Some(item) => self.print(&item.full),
            None => {
                let items = self.fmt_items();
                self.print(&items);
            }
        };
    }
    fn fmt_items(&self) -> String {
        let mut buffer = String::new();
        for (i, item) in self.items.iter().enumerate() {
            let title: String;
            if i == self.current_item {
                title = format!("{}{}{}", color::Fg(color::Blue), item.title, style::Reset);
            } else {
                title = item.title.clone();
            }
            buffer = format!("{}{}\r\n", buffer, title);
        }

        buffer = buffer.trim_right_matches("\r\n").to_string();
        buffer
    }
    fn listen_keys(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Up => {
                    if self.current_item > 0 {
                        self.current_item -= 1
                    }
                }
                Key::Down => {
                    if self.current_item < self.items.len() - 1 {
                        self.current_item += 1
                    }
                }
                Key::Char('\n') => {
                    self.openned_item = Some(self.items[self.current_item].clone());
                }
                Key::Char('q') => break,
                Key::Ctrl('c') => break,
                Key::Esc => self.openned_item = None,
                Key::Left => self.openned_item = None,
                _ => {}
            }
            stdout.flush().unwrap();
            self.redraw();
        }
    }
}

fn line_count(buffer: &str) -> u16 {
    buffer.rmatches("\r\n").count() as u16
}
