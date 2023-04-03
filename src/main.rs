use std::env::args;
use std::fs;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut files = vec![];
    let args: Vec<_> = args().collect();
    let mut options = Options::new();

    for arg in args.iter().skip(1) {
        if &arg[..] == "-" {
            files.push(read_stdin());
            continue;
        }

        if arg.starts_with("-") && options.parse(&arg) {
            //
        } else {
            match fs::read(&arg) {
                Ok(f) => files.push(f),
                Err(e) => eprintln!("Err: reading file at [{arg}] err: [{e}]"),
            }
        }
    }

    // eprintln!("{:?}", options);

    let mut stdout = stdout().lock();
    for f in &files {
        let mut output: Vec<u8> = vec![];

        for byte in f {
            match byte {
                b'\t' => {
                    if options.show_tabs {
                        output.push(b'^');
                        output.push(b'I');
                    } else {
                        output.push(b'\t');
                    }
                }
                b'\n' => output.push(b'\n'),
                _ => output.push(*byte),
            }
        }

        if let Err(e) = stdout.write(&output[..]) {
            eprintln!("Err: writing file, err: [{e}]");
        }
    }
}

#[derive(Debug)]
pub struct Options {
    show_all: bool,        // -A --show-all
    number_nonblank: bool, // -b, number-nonblank , overrides number
    show_ends: bool,       // -E --show-ends
    number: bool,          // -n, --number
    squeeze_blank: bool,   // -s, --squeeze-blank
    show_tabs: bool,       // -T, --show-tabs
}

impl Options {
    pub fn new() -> Self {
        Self {
            show_all: false,
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
            "-A" | "--show-all" => self.show_all = true,
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

fn read_stdin() -> Vec<u8> {
    let mut buf = Vec::new();
    let mut stdin = stdin().lock();

    // TODO: handle err
    stdin.read_to_end(&mut buf).unwrap();

    buf
}
