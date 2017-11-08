use termion::{color, style, clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use time;

use std::io::{Write, stdout, stdin};

use crates::Krate;
use error::Error;


#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub full: String,
}
pub struct Cli {
    items: Vec<Item>,
    current_item: usize,
    openned_item: Option<Item>,
}
impl Cli {
    pub fn new() -> Cli {
        Cli {
            items: Vec::new(),
            current_item: 0,
            openned_item: None,
        }
    }
    pub fn print(&mut self, print: &str) {
        let print = format!("{}\r", print);
        print!("{}", print);
    }
    pub fn print_items(&mut self, items: Vec<Item>) {
        self.items = items;
        self.redraw();
        self.listen_keys();
    }
    fn clear(&self) {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
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
                title = format!(
                    "{}{}{}",
                    color::Fg(color::Blue),
                    item.title.clone(),
                    style::Reset
                );
            } else {
                title = item.title.clone();
            }
            buffer += &format!("{}\r\n", title.replace("\n", ""));
        }

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
                Key::Left | Key::Esc => self.openned_item = None,
                Key::Char('\n') | Key::Right => self.open_current_item(),
                Key::Char('q') | Key::Ctrl('c') => break,
                _ => {}
            }
            stdout.flush().unwrap();
            self.redraw();
        }
    }
    fn open_current_item(&mut self) {
        self.openned_item = Some(self.items[self.current_item].clone());
    }
}

pub fn fmt_krate(krate: Krate) -> Result<String, Error> {
    let line = |key: &str, val: &str| {
        format!(
            "{}{}:{} {}\r\n",
            color::Fg(color::Blue),
            key,
            style::Reset,
            val.replace("\n", "")
        )
    };

    let mut fmt = String::new();
    fmt += &line("Name", &krate.name);
    fmt += &line("Description", &krate.description);
    fmt += &line("Last version", &krate.max_version);

    if let Some(license) = krate.license {
        fmt += &line("License", &license)
    }
    if let Some(homepage) = krate.homepage {
        fmt += &line("Home page", &homepage)
    }
    if let Some(doc) = krate.documentation {
        fmt += &line("License", &doc)
    }
    if let Some(repository) = krate.repository {
        fmt += &line("Repository", &repository)
    }

    fmt += &line("Created", &parse_time(krate.created_at)?);
    fmt += &line("Updated", &parse_time(krate.updated_at)?);

    Ok(fmt)
}

fn parse_time(t: String) -> Result<String, Error> {
    let parsed = time::strptime(&t, "%Y-%m-%dT%H:%M:%S")?;
    Ok(time::strftime("%Y-%m-%d %H:%M:%S", &parsed)?)
}
