use std::path::PathBuf;

use crate::lexer::Scanner;
pub fn evaluate_file(f: &PathBuf) {
    let contents =
        std::fs::read_to_string(f).unwrap_or_else(|_| panic!("Could not find file: {:?}", f));
    run(contents.as_str());
}

pub fn run(source: &str) {
    let scanner = &mut Scanner::default();
    let tokens = scanner.scan_tokens(source.to_string());

    match tokens {
        Ok(t) => {
            for token in t {
                println!(
                    "[{}, {}]",
                    std::str::from_utf8(&token.lexeme).unwrap(),
                    token.ttype,
                );
            }
        }
        Err(e) => {
            for er in e {
                println!("{}", er)
            }
        }
    }
}
