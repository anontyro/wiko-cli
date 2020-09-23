use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let total = read_file(&args.path, &args.pattern);
    println!("pattern: {} is present: {} times", args.pattern, total)
}

fn read_file(path: &PathBuf, pattern: &str) -> i32 {
    let mut total = 0;

    let f = File::open(&path).expect("error");
    let mut reader = BufReader::new(f);

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line).expect("error reading line");
        if len == 0 {
            break;
        }
        if line.contains(pattern) {
            total += 1;
        }
    }
    total
}
