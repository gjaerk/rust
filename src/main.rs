
// grep-lite tool
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};

// The process_lines function takes a reader (either from a file or 
// from standard input) and a regular expression, and for each line 
// it finds a match in the line, it prints the line. 
fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

// The main function sets up the command line arguments using Clap, 
// compiles the regular expression from the pattern argument, and 
// determines whether the input is from a file or from standard input. 
// If it's from a file, it opens the file, creates a reader from the 
// file, and passes the reader and the compiled regular expression to 
// process_lines. If the input is from standard input, it creates a 
// reader from standard input and passes that to process_lines.
fn main() {
    let args = App::new("grep-lite")
    .version("0.1")
    .about("Search for patterns")
    .arg(Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true))
    .arg(Arg::with_name("input")
        .help("File to search")
        .takes_value(true)
        .required(true))
    .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
