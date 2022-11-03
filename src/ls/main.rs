use std::fs;

fn main() {
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        // remove ./
        let path = path.unwrap().path().to_str().unwrap().to_string();
        let path = path[2..].to_string();
        println!("{}", path)
    }
}
