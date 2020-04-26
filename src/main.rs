use std::env;
use std::path::Path;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    match path {
        Some(path) => print_files(path),
        None => println!("ERROR: Enter a path"),
    }
}

fn print_files(path: &str) {
    let entries = lib::list_dir(Path::new(path));
    for entry in entries {
        println!("{}", entry.name)
    }
}