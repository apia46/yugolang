use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command()]
enum Args {
    Input {
        #[arg(short)]
        input:String
    },
    File {
        #[arg(short)]
        path: PathBuf
    },
}

fn main() {
    let args = Args::parse();
    let input = match args {
        Args::Input{input} => input,
        Args::File{path} => fs::read_to_string(path).unwrap(),
    };
    let parse = yugolang_parser::parse(&input).unwrap();
    print!("{parse:?}");
}

