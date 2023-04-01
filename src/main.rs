use std::env::args;
use std::fs;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut files = vec![];
    for arg in args().skip(1) {
        if &arg[..] == "-" {
            let mut stdin = stdin().lock();
            let mut buffer = Vec::new();

            if let Ok(_) = stdin.read_to_end(&mut buffer) {
                files.push(buffer);
            }
        } else {
            match fs::read(&arg) {
                Ok(f) => files.push(f),
                Err(e) => eprintln!("Err: reading file at [{arg}] err: [{e}]"),
            }
        }
    }

    let mut stdout = stdout().lock();
    for f in files {
        if let Err(e) = stdout.write(&f[..]) {
            eprintln!("Err: writing file, err: [{e}]");
        }
    }
}
