use chrono::DateTime;
use std::env;
use std::fs;

extern crate chrono;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    match path {
        Some(path) => list_contents(path),
        None => println!("ERROR: Enter a path"),
    }
}

fn list_contents(path: &String) {
    let metadata = fs::metadata(path);
    match metadata {
        Err(e) => print!("ERROR: {}", e),
        Ok(metadata) => {
            if metadata.file_type().is_dir() {
                list_dir(path)
                    .iter()
                    .for_each(|output| println!("{}", output))
            } else {
                println!("ERROR: {} is not a directory", path)
            }
        }
    }
}

fn list_dir(path: &String) -> Vec<String> {
    let mut outputs = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let dir_entry = entry.unwrap();
        let metadata = dir_entry.metadata().unwrap();
        let accessed_time = metadata.accessed().unwrap();
        let date_time: DateTime<chrono::Local> = accessed_time.into();
        let output = format!(
            "{:20} {}",
            dir_entry.file_name().to_str().unwrap(),
            date_time.format("%b %e %H:%M").to_string()
        );
        outputs.push(output);
    }
    return outputs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dir() {
        let representation = list_dir(&String::from("./src/test_dir"));
        assert_eq!(representation.len(), 2)
    }
}
