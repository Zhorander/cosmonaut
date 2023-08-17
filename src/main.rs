use clap::Parser;

pub mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CosmonautArgs {
    #[arg(short, long)]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = CosmonautArgs::parse();
    println!("{:?}", args);

    if let Err(err) = parser::parse_source("let x = 23 in true") {
        println!("{}", err)
    }
}
