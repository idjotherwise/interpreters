use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,
}

fn main() {
    let file = Cli::parse();
    match file.path {
        None => println!("Running repl..."),
        Some(f) => println!("Executing {:?}", f),
    };
}
