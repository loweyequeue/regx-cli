use clap::Parser;
use once_cell::sync::Lazy;
use std::io::{self, BufRead, Write};

#[derive(Debug, Parser)]
#[command(name = "regex")]
#[command(about = "A tool to run regexes over stdin")]
struct Cli {
    #[arg(short, long, conflicts_with = "index")]
    label: Option<Vec<String>>,
    #[arg(short, long, conflicts_with = "label")]
    index: Option<Vec<usize>>,

    #[arg(required = true)]
    regex: String,
}

static STDOUT: Lazy<io::Stdout> = Lazy::new(io::stdout);

fn print_match<D: std::fmt::Display>(m: D) {
    let mut stdout = STDOUT.lock();
    if let Err(e) = writeln!(stdout, "{}", m) {
        if e.kind() == io::ErrorKind::BrokenPipe {
            std::process::exit(0);
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let rx = regex::Regex::new(&cli.regex).unwrap_or_else(|e| {
        eprintln!("Invalid regex: {e:#?}");
        std::process::exit(1);
    });

    let stdin = io::stdin();

    for line in stdin.lock().lines().map_while(Result::ok) {
        if cli.label.is_some() || cli.index.is_some() {
            // use capture groups
            match rx.captures(&line) {
                Some(groups) => {
                    if let Some(labels) = &cli.label {
                        for label in labels {
                            if let Some(group) = groups.name(label) {
                                print_match(group.as_str());
                            }
                        }
                    } else if let Some(indexes) = &cli.index {
                        for index in indexes {
                            if let Some(group) = groups.get(*index + 1) {
                                print_match(group.as_str());
                            }
                        }
                    }
                }
                None => continue,
            }
        } else {
            // no capture groups, only matches
            let x = rx.find(&line);
            if let Some(m) = x {
                print_match(m.as_str());
            }
        }
    }
}
