mod slice;

use clap::Parser;
use slice::SliceSpec;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
struct Args {
    /// Number all output lines
    #[arg(short, long, default_value_t = false)]
    number: bool,

    /// Path to the file with the lines range to display
    #[arg(
        trailing_var_arg(true),
        value_name = "file[:start[:end]]",
        required = true
    )]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let printer = if args.number {
        |(idx, line)| println!("{}\t{}", idx + 1, line)
    } else {
        |(_, line)| println!("{}", line)
    };

    for path in args.files {
        let slice = SliceSpec::from(path.as_str());
        let reader = match File::open(&slice.filename) {
            Ok(file) => BufReader::new(file),
            Err(_) => {
                eprintln!("cannot open file: {}", &slice.filename);
                std::process::exit(1)
            }
        };

        let start = slice.start.unwrap_or(1);
        let end = slice.end.unwrap_or(usize::MAX);
        if start > end {
            eprintln!("invalid slice range: {} > {}", start, end);
            std::process::exit(1)
        }

        let _ = reader
            .lines()
            .enumerate()
            .filter_map(|(idx, line)| {
                match line {
                    Ok(line) => Some((idx, line)),
                    Err(err) => {
                        // print errors to stderr and carry on
                        eprintln!("{}", err);
                        None
                    }
                }
            })
            .take(end)
            .skip(start - 1)
            .for_each(printer);
    }
}
