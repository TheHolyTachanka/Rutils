use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cat <file>");
        return;
    }
    println!("{}", fs::read_to_string(args[1].clone()).unwrap())
}