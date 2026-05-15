#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut commands = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut commands) {
            Ok(_) => {
                println!("{}: command not found", commands.trim());
            }
            Err(_) => {}
        }
    }
}
