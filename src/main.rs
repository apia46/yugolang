use clap::Parser;
mod lexer;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    input:String,
}

fn main() {
    let args = Args::parse();
    let result = lexer::tokenize(&args.input).unwrap();
    print!("{result:?}");
}
