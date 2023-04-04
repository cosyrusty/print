use std::env::args;
use std::io::{stdin, stdout, Read, Write};
use std::{fs, io};

fn main() {
    let mut files = vec![];
    let args: Vec<_> = args().collect();
    let mut options = Options::new();

    for arg in args.iter().skip(1) {
        if &arg[..] == "-" {
            match read_stdin() {
                Ok(f) => files.push(f),
                Err(e) => eprintln!("Err: reading stdin, err: [{e}]"),
            }
            continue;
        }

        if arg.starts_with("-") && options.parse(&arg) {
            //
        } else {
            match read_file(arg) {
                Ok(f) => files.push(f),
                Err(e) => eprintln!("Err: reading file at [{arg}] err: [{e}]"),
            }
        }
    }

    // eprintln!("{:?}", options);

    let mut ln = 1; // current line number
    for f in &files {
        print_file(&f, &options, &mut ln).unwrap()
    }
}

fn print_file(file: &Vec<Line>, _options: &Options, _ln: &mut usize) -> io::Result<()> {
    let mut stdout = stdout().lock();
    for line in file {
        stdout.write_all(line.as_bytes())?
    }

    Ok(())
}

#[derive(Debug)]
pub struct Options {
    number_nonblank: bool, // -b, number-nonblank , overrides number
    show_ends: bool,       // -E --show-ends
    number: bool,          // -n, --number
    squeeze_blank: bool,   // -s, --squeeze-blank
    show_tabs: bool,       // -T, --show-tabs
}

impl Options {
    pub fn new() -> Self {
        Self {
            number_nonblank: false,
            show_ends: false,
            number: false,
            squeeze_blank: false,
            show_tabs: false,
        }
    }

    // true if successfully parsed else false
    pub fn parse(&mut self, arg: &str) -> bool {
        match arg {
            "-b" | "--number-nonblank" => self.number_nonblank = true,
            "-E" | "--show-ends" => self.show_ends = true,
            "-n" | "--number" => self.number = true,
            "-s" | "--squeeze-blank" => self.squeeze_blank = true,
            "-T" | "--show-tabs" => self.show_tabs = true,
            _ => {
                false;
            }
        }
        true
    }
}

type Line = String;

fn read_stdin() -> io::Result<Vec<Line>> {
    let mut buf = String::new();
    let mut stdin = stdin().lock();

    stdin.read_to_string(&mut buf)?;

    Ok(buf
        .split_inclusive("\n")
        .map(|str| str.to_string())
        .collect())
}

fn read_file(path: &str) -> io::Result<Vec<Line>> {
    Ok(fs::read_to_string(path)?
        .split_inclusive("\n")
        .map(|str| str.to_string())
        .collect())
}
