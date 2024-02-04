use std::io::Write;

use crate::eval::run;

pub fn start_repl() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("Welcome to rlox! Version {}", VERSION);
    loop {
        let mut inp = String::new();
        print!("rlox> ");
        let _ = std::io::stdout().flush();
        std::io::stdin()
            .read_line(&mut inp)
            .expect("Could not read input");
        if inp == ":q\n" {
            println!("Bye!");
            break;
        }
        if let Some('\n') = inp.chars().next_back() {
            inp.pop();
        }
        if let Some('\r') = inp.chars().next_back() {
            inp.pop();
        }
        run(inp.as_str());
    }
}
