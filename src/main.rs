
use std::io::{self, Write};


fn main() {
    const SHELL: &str = "rush";
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    loop {
         input.clear();
         print!("{SHELL}> ");
         stdout.flush().unwrap();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }
        println!("{input}");
    }
}
