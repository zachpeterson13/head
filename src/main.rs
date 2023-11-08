use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, Read};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None,
override_usage(
        "head [OPTIONS]... [FILE]... \n\
         Print the first 10 lines of each FILE to standard output. \n\
         With more than on FILE, precede each with a header giving the file name. \n\n\
         With no FILE, or when FILE is -, read standard input.",
    ),
after_help(
        "NUM may have a multiplier suffix: \n\
b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024, \n\
GB 1000*1000*1000, G 1024*1024*1024",
))]
struct Cli {
    /// Print the first NUM bytes of each file;
    ///   with the leading '-', print all but the last
    ///   NUM bytes of each file
    #[arg(short = 'c', value_name("[-]NUM"), long, verbatim_doc_comment)]
    bytes: Option<String>,

    /// Print the first NUM lines instead of the frist 10;
    ///   with the leading '-', print all the but the last
    ///   NUM lines of each file
    #[arg(
        short = 'n',
        long,
        value_name("[-]NUM"),
        default_value = "10",
        verbatim_doc_comment
    )]
    lines: Option<isize>,

    /// Never print headers giving file names
    #[arg(long, short)]
    quiet: bool,

    /// Always print headers giving file names
    #[arg(long, short)]
    verbose: bool,

    /// Line delimiter is NUL, not newline
    #[arg(long, short)]
    zero_terminated: bool,

    #[arg()]
    /// The file(s) to print
    file: Option<Vec<String>>,
}

fn process(
    reader: &mut impl io::BufRead,
    is_bytes: bool,
    line_end: bool,
    c: Option<isize>,
    n: Option<isize>,
) -> Result<()> {
    if is_bytes {
        let bytes: Vec<_> = reader.bytes().collect();
        let count = bytes.len();

        let c = c.unwrap();
        let c = if c < 0 {
            count - c.unsigned_abs()
        } else {
            c.try_into().unwrap()
        };

        process_bytes(bytes, c);
    } else {
        let lines: Vec<_> = reader.lines().collect();
        let count = lines.len();

        let n = n.unwrap();
        let n = if n < 0 {
            count - n.unsigned_abs()
        } else {
            n.try_into().unwrap()
        };

        process_lines(lines, n, line_end);
    }

    Ok(())
}

fn process_lines(lines: Vec<Result<String, io::Error>>, line_count: usize, line_end: bool) {
    for (i, line) in lines.into_iter().enumerate() {
        if i == line_count && !line_end {
            break;
        }

        if let Ok(line) = line {
            println!("{}", line);
        }
    }
}

fn process_bytes(bytes: Vec<Result<u8, io::Error>>, byte_count: usize) {
    for (i, byte) in bytes.into_iter().enumerate() {
        if i == byte_count {
            break;
        }
        if let Ok(byte) = byte {
            print!("{}", byte as char);
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let filenames = cli.file.unwrap_or(vec![String::from("-")]);

    let mut print_headers = filenames.len() > 1;
    if cli.verbose {
        print_headers = true;
    }
    if cli.quiet {
        print_headers = false;
    }

    let line_end = cli.zero_terminated;
    let is_bytes = cli.bytes.is_some();

    let bytes = parse_bytes(cli.bytes);

    for (i, filename) in filenames.into_iter().enumerate() {
        if print_headers {
            if i > 0 {
                println!();
            }
            println!(
                "==> {} <==",
                if filename == "-" {
                    "standard input"
                } else {
                    &filename
                }
            );
        }

        if filename == "-" {
            let mut reader = io::BufReader::new(io::stdin().lock());
            process(&mut reader, is_bytes, line_end, bytes, cli.lines)?;
        } else {
            let file = File::open(filename)?;
            let mut reader = io::BufReader::new(file);
            process(&mut reader, is_bytes, line_end, bytes, cli.lines)?;
        }
    }

    Ok(())
}

fn parse_bytes(input: Option<String>) -> Option<isize> {
    let input = input?;

    let mut idx = 0;

    for char in input.chars() {
        if !char.is_numeric() && char != '-' {
            break;
        }

        idx += 1;
    }

    let (num_str, multiplier) = input.split_at(idx);

    let num: isize = num_str.parse().expect("Number of bytes must be a number");

    // NUM may have a multiplier suffix:
    // b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,
    // GB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.
    Some(
        num * match multiplier {
            "b" => 512_isize,
            "kB" => 1000_isize.pow(1),
            "K" => 1024_isize.pow(1),
            "MB" => 1000_isize.pow(2),
            "M" => 1024_isize.pow(2),
            "GB" => 1000_isize.pow(3),
            "G" => 1024_isize.pow(3),
            // "TB" => 1000_isize.pow(4),
            // "T" => 1024_isize.pow(4),
            // "PB" => 1000_isize.pow(5),
            // "P" => 1024_isize.pow(5),
            // "EB" => 1000_isize.pow(6),
            // "E" => 1024_isize.pow(6),
            // "ZB" => 1000_isize.pow(7),
            // "Z" => 1024_isize.pow(7),
            // "YB" => 1000_isize.pow(8),
            // "Y" => 1024_isize.pow(8),
            _ => panic!("Invalid Multiplier"),
        },
    )
}
