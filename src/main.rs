use std::io::{self, BufRead, Write};

use clap::Parser;
use once_cell::sync::Lazy;
use regex::Regex;

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

enum RegexMode {
    Simple,
    Extract,
}

impl Cli {
    fn mode(&self) -> RegexMode {
        if self.label.is_some() || self.index.is_some() {
            RegexMode::Extract
        } else {
            RegexMode::Simple
        }
    }
}

// Lazy to maybe improve performance but it was probably already optimized by the rustc compiler
static STDOUT: Lazy<io::Stdout> = Lazy::new(io::stdout);

/// Fn to print matches and handle BrokenPipe i.e when output is piped through `head`
fn print_match<D: std::fmt::Display>(m: D) {
    let mut stdout = STDOUT.lock();
    if let Err(e) = writeln!(stdout, "{}", m) {
        if e.kind() == io::ErrorKind::BrokenPipe {
            std::process::exit(0);
        }
    }
}

/// Simple search, print any line that matches
fn search_simple(rx: Regex, stdin: io::Stdin) {
    for line in stdin.lock().lines().map_while(Result::ok) {
        if let Some(m) = rx.find(&line) {
            print_match(m.as_str());
        }
    }
}

/// Search and extract data using capture groups
fn search_extract(rx: Regex, stdin: io::Stdin, cli: Cli) {
    for line in stdin.lock().lines().map_while(Result::ok) {
        if let Some(groups) = rx.captures(&line) {
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
    }
}

fn main() {
    let cli = Cli::parse();
    let rx = Regex::new(&cli.regex).unwrap_or_else(|e| {
        eprintln!("Invalid regex: {e:#?}");
        std::process::exit(1);
    });

    let stdin = io::stdin();

    match cli.mode() {
        RegexMode::Simple => search_simple(rx, stdin),
        RegexMode::Extract => search_extract(rx, stdin, cli),
    }
}
