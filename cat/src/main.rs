use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", fs::read_to_string(args[1].clone()).unwrap())
}
