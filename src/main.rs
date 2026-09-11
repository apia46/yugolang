use clap::Parser;

mod lexer;
mod parser;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    input:String,
}

fn main() {
    let args = Args::parse();
    let tokens = lexer::tokenize(&args.input).unwrap();
    let parse = parser::parse(tokens).unwrap();
    print!("{parse:?}");
}
