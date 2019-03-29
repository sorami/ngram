use std::io::Write;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process;

use structopt::StructOpt;

/// Count n-grams in text.
#[derive(StructOpt)]
#[structopt(name = "ngram", author = "")]
struct Cli {
    /// "n" of "n"-grams
    n: usize,

    #[structopt(parse(from_os_str))]
    /// Input file: If not specified, read from stdin
    file: Option<PathBuf>,

    #[structopt(parse(from_os_str), short = "o", long = "out")]
    /// Output file: If not specified, write to stdout
    out_file: Option<PathBuf>,

    /// Reverse to the ascending order
    #[structopt(short = "r", long = "reverse")]
    rev: bool,

    /// Add "<BOS>" and "<EOS>" to the beginning and the end of line
    #[structopt(short = "p", long = "padding")]
    padding: bool,

    /// Token delimiter. Default is " " (whitespace)
    #[structopt(short = "d", long = "delim")]
    string: Option<String>,
}

fn main() {
    let args = Cli::from_args();
    let n = args.n;

    let delim = match args.string {
        Some(s) => s,
        None => " ".to_string(),
    };

    // input: stdin or file
    let reader: Box<BufRead> = match args.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(input_path) => Box::new(BufReader::new(
            match File::open(input_path) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("error: Input: {}", e);
                    process::exit(1);
                },
            }
        )),
    };

    let mut counts = HashMap::new();

    for line in reader.lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => {
                eprintln!("error: Reading from input: {}", e);
                process::exit(1);
            }
        };

        let mut tokens: Vec<&str> = line.split(&delim).collect();

        if args.padding {
            tokens.insert(0, "<BOS>");
            tokens.push("<EOS>");
        }

        if tokens.len() < n {
            continue;
        }

        let end = tokens.len() - n + 1;
        for i in 0..end {
            let slice = &tokens[i..i + n];
            let ngram = slice.join(" ");
            let entry = counts.entry(ngram).or_insert(0);
            *entry += 1;
        }
    }

    let mut counts_vec: Vec<_> = counts.iter().collect();
    if !args.rev {
        // descending
        counts_vec.sort_by(|a, b| a.0.cmp(b.0)); // sort alphabetically
        counts_vec.sort_by(|a, b| b.1.cmp(a.1)); // sort by counts
    } else {
        // ascending
        counts_vec.sort_by(|b, a| a.0.cmp(b.0)); // sort alphabetically
        counts_vec.sort_by(|b, a| b.1.cmp(a.1)); // sort by counts
    };

    // output: stdout or file
    let mut writer = match args.out_file {
        None => Box::new(io::stdout()),
        Some(p) => {
            Box::new(match File::create(&p) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("error: Output: {}", e);
                    process::exit(1);
                }
            }) as Box<Write> 
        }
    };

    for (ngram, c) in counts_vec {
        match write!(writer, "{}\t{}\n", c, ngram) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("error: Writing to output: {}", e);
                process::exit(1);
            }
        }
    }
}
