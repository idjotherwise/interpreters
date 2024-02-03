use std::io::Write;

pub fn start_repl() {
    let mut inp = String::new();
    print!("rlox> ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Could not read input");
    if let Some('\n') = inp.chars().next_back() {
        inp.pop();
    }
    if let Some('\r') = inp.chars().next_back() {
        inp.pop();
    }
    println!("{}", inp);
}
