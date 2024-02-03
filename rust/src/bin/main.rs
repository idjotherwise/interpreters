use clap::Parser;

use rlox::eval;
use rlox::repl;

#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,
}

fn main() {
    let file = Cli::parse();
    match file.path {
        None => repl::start_repl(),
        Some(f) => eval::evaluate_file(&f),
    };
}
