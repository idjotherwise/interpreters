use std::path::PathBuf;

use crate::lexer::Scanner;
pub fn evaluate_file(f: &PathBuf) {
    let contents =
        std::fs::read_to_string(f).expect(format!("Could not find file: {:?}", f).as_str());
    run(contents.as_str());
}

pub fn run(source: &str) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!(
            "[{}, {}, (line: {})]",
            token.literal, token.ttype, token.line
        )
    }
}
