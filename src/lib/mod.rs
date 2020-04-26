use std::fs;
use std::path::Path;

pub struct Entry {
    pub name: String
}

pub fn list_dir(path: &Path) -> Vec<Entry> {
    let mut entries = Vec::new();
    if let Ok(read_dir_res) = fs::read_dir(path) {
        for dir_entry_res in read_dir_res {
            if let Ok(dir_entry) = dir_entry_res {
                let os_file_name = dir_entry.file_name();
                if let Ok(file_name) = os_file_name.into_string() {
                    entries.push(Entry { name: file_name })
                }
            }
        }
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_files() {
        let entries = list_dir(Path::new("./src/lib/test_dir"));

        assert_eq!(entries.len(), 2);
        assert_eq!(entries.get(0).unwrap().name, "file.txt");
        assert_eq!(entries.get(1).unwrap().name, "image.jpg");
    }
}