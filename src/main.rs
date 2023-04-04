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
            if options.help {
                print_help().unwrap();
                return;
            }
            if options.version {
                print_version().unwrap();
                return;
            }
        } else {
            match read_file(arg) {
                Ok(f) => files.push(f),
                Err(e) => eprintln!("Err: reading file at [{arg}] err: [{e}]"),
            }
        }
    }

    // eprintln!("{:?}", options);

    let mut ln = 0; // current line number
    for f in &files {
        print_file(&f, &options, &mut ln).unwrap()
    }
}

fn print_help() -> io::Result<()> {
    let mut stdout = stdout().lock();
    stdout.write_all("print [FILE]...\n".as_bytes())?;
    Ok(())
}

fn print_version() -> io::Result<()> {
    let mut stdout = stdout().lock();
    stdout.write_all("v: 0.1\n".as_bytes())?;
    Ok(())
}

fn print_file(file: &Vec<Line>, options: &Options, ln: &mut usize) -> io::Result<()> {
    let mut stdout = stdout().lock();
    let mut empty_line = false;

    'a: for line in file {
        let mut nline = Line::new();

        if line.starts_with('\n') {
            // previous line was already an empty line then
            // handle -s arg
            if empty_line && options.squeeze_blank {
                continue 'a;
            }
            empty_line = true;
        } else {
            empty_line = false;
        }

        // if line is not squeezed than increment the line number
        *ln += 1;

        if options.number_nonblank {
            // handle -b arg
            if empty_line {
                // this line does not count hence decrement line number
                *ln -= 1;
                let eight_spaces = "        ";
                nline.push_str(&eight_spaces)
            } else {
                nline.push_str(&format!("{:>6}  ", *ln))
            }
        } else if options.number {
            // handle -n arg
            nline.push_str(&format!("{:>6}  ", *ln))
        }

        for char in line.chars() {
            if char == '\n' && options.show_ends {
                nline.push_str("$\n")
            } else if char == '\t' && options.show_tabs {
                nline.push_str("^I")
            } else {
                nline.push(char)
            }
        }

        stdout.write_all(nline.as_bytes())?
    }

    Ok(())
}

#[derive(Debug)]
pub struct Options {
    help: bool,
    version: bool,
    number_nonblank: bool, // -b, number-nonblank , overrides number
    show_ends: bool,       // -E --show-ends
    number: bool,          // -n, --number
    squeeze_blank: bool,   // -s, --squeeze-blank
    show_tabs: bool,       // -T, --show-tabs
}

impl Options {
    pub fn new() -> Self {
        Self {
            help: false,
            version: false,
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
            "--help" => self.help = true,
            "--version" => self.version = true,
            "-b" | "--number-nonblank" => self.number_nonblank = true,
            "-e" | "--show-ends" => self.show_ends = true,
            "-n" | "--number" => self.number = true,
            "-s" | "--squeeze-blank" => self.squeeze_blank = true,
            "-t" | "--show-tabs" => self.show_tabs = true,
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
