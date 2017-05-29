#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;
extern crate termion;
extern crate time;

use std::env;
use std::process;

mod cli;
mod error;
mod crates;
use cli::{Cli, Item};

fn main() {
    let mut cli = Cli::new();
    let query = match env::args().nth(2) {
        Some(query) => query,
        None => {
            println!("Use: cargo find <query>");
            process::exit(2);
        }
    };

    cli.print("Searching....");
    let crates = crates::find_crates(query).unwrap();
    let mut items = Vec::new();
    for krate in crates {
        items.push(Item {
            title: format!("{} - {}", krate.name, krate.description),
            full: cli::fmt_krate(krate).unwrap(),
        });
    }

    if items.len() < 1 {
        cli.print("No results");
    } else {
        cli.print_items(items);
    }
}
