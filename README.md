# regx-cli

A tool similar to grep but it only supports regex.

Next to that it allow direct extraction of data using (named) match groups.

## Installation

```bash
git clone https://github.com/loweyequeue/regx-cli.git
cd regx-cli
cargo install --path .
```

## Usage

```bash
echo "Hello, Rust!" | regx "Hello, (\w+)!"
# >> Hello, Rust!
echo "Hello, Rust!" | regx --index 0 "Hello, (\w+)!"
# >> Rust
echo "Hello, Rust!" | regx --label language "Hello, (?P<language>\w+)!"
# >> Rust
echo "Hello, Rust!" | regx -i0 -i1 "Hello, (\w+)(!)"
# >> Rust
# >> !
```

## Why

You may wonder why does this exist, we have grep, awk, sed, etc

If you rarely use those commands its way easier to create a quick regex since they are dead simple.

Some existing tools were way too much (code) for a simple data in data out task like this so I made my own.
