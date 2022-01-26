use std::{fs, io};
use std::io::{BufRead, Write};
use clap::Arg;


const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");


fn main() {
    let args = clap::App::new(NAME)
        .version(VERSION)
        .arg(Arg::new("file"))
        .get_matches();

    let reader: Box<dyn io::BufRead> = if let Some(file) = args.value_of("file") {
        Box::new(io::BufReader::new(fs::File::open(file).unwrap()))
    } else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    for line in reader.lines() {
        stdout.write(&*strip_ansi_escapes::strip(&line.unwrap().as_bytes()).unwrap());
        stdout.write(b"\n");
    }
}
