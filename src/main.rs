extern crate termion;

mod cli;
use cli::{Cli, Item};

fn main() {
    let mut cli = Cli::new();
    let krates = vec![Item {
                          title: "First item".to_string(),
                          full: "Expanded first".to_string(),
                      },
                      Item {
                          title: "Second item".to_string(),
                          full: "Expanded two".to_string(),
                      },
                      Item {
                          title: "Thi item".to_string(),
                          full: "Expanded tri".to_string(),
                      },
                      Item {
                          title: "Thi item".to_string(),
                          full: "Expanded tri".to_string(),
                      },
                      Item {
                          title: "Thi item".to_string(),
                          full: "Expanded tri".to_string(),
                      },
                      Item {
                          title: "Thi item".to_string(),
                          full: "Expanded tri".to_string(),
                      }];

    cli.print("Searching....");
    std::thread::sleep_ms(1000);
    cli.print_items(krates);
}
