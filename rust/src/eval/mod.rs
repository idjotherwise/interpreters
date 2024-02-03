use std::path::PathBuf;

pub fn evaluate_file(f: &PathBuf) {
    let contents =
        std::fs::read_to_string(f).expect(format!("Could not find file: {:?}", f).as_str());
    println!("{}", contents);
}
