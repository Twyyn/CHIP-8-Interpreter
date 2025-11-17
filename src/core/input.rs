use inquire::{Select, error::InquireError};
use std::ffi::OsString;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Directory {
    files: Vec<OsString>,
}
impl Directory {
    pub fn new() -> Directory {
        Directory { files: Vec::new() }
    }
    fn walk(&mut self) {
        for entry in WalkDir::new("test_rom/").into_iter().filter_map(|e| e.ok()) {
            self.files.push(entry.path().to_owned().into());
        }
    }
    pub fn select(&mut self) -> Option<&OsString> {
        self.walk();
        let options: Vec<&str> = self
            .files
            .iter()
            .map(|p| std::path::Path::new(p).file_stem().unwrap())
            .filter_map(|s| s.to_str())
            .collect();

        let ans: Result<&str, InquireError> = Select::new("Select Game", options).prompt();

        match ans {
            Ok(selected_name) => {
                let m = self
                    .files
                    .iter()
                    .find(|&path| {
                        std::path::Path::new(path)
                            .file_stem()
                            .and_then(|s| s.to_str())
                            == Some(selected_name)
                    })
                    .unwrap();
                println!("{:?}", m);
                Some(m)
            }
            Err(_) => {
                println!("There was an error, please try again");
                None
            }
        }
    }
}
