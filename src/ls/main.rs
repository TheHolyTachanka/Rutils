use std::{fs, env};

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        list_dir(".");
        return;
    }else if args[1] == "--help" || args[1] == "-h" {
        println!("Usage: ls <dir>")
    }else {
        list_dir(&args[1])
    }
    
}

fn list_dir(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        // remove ./
        let path = path.unwrap().path().to_str().unwrap().to_string();
        let path = path[2..].to_string();
        println!("{}", path)
    }

}
