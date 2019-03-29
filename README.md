# ngram

A small CLI program to count n-gram. Written in Rust.

```
ngram 0.1.0
Count n-grams in text.

USAGE:
    ngram [FLAGS] [OPTIONS] <n> [file]

FLAGS:
    -h, --help       Prints help information
    -p, --padding    Add "<BOS>" and "<EOS>" to the beginning and the end of line
    -r, --reverse    Reverse to the ascending order
    -V, --version    Prints version information

OPTIONS:
    -o, --out <out_file>    Output file: If not specified, write to stdout
    -d, --delim <string>    Token delimiter. Default is " " (whitespace)

ARGS:
    <n>       "n" of "n"-grams
    <file>    Input file: If not specified, read from stdin
```

## Installation

Compiling from this repository:

```
$ git clone git://github.com/sorami/ngram
$ cd ngram
$ cargo build --release

# compiled binary will be ./taret/release/ngram
```

You will need [Cargo](https://doc.rust-lang.org/stable/cargo/), the Rust package manager.

