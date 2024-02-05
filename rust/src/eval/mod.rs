use std::path::PathBuf;

use crate::lexer::Scanner;
pub fn evaluate_file(f: &PathBuf) {
    let contents =
        std::fs::read_to_string(f).expect(format!("Could not find file: {:?}", f).as_str());
    run(contents.as_str());
}

pub fn run(source: &str) {
    let scanner = &mut Scanner::default();
    let tokens = scanner.scan_tokens(source.to_string());

    for token in tokens.unwrap() {
        println!(
            "[{}, {}, (line: {})]",
            std::str::from_utf8(&token.lexeme).unwrap(),
            token.ttype,
            token.line
        );
    }
}
