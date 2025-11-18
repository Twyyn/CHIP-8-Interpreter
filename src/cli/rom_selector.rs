use inquire::{Select, error::InquireError};
use std::ffi::OsString;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct RomSelector {
    files: Vec<OsString>,
}
impl RomSelector {
    pub fn new() -> RomSelector {
        RomSelector { files: Vec::new() }
    }

    fn walk(&mut self) {
        self.files.clear(); // in case select() is called more than once

        let rom_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/roms");

        for entry in WalkDir::new(&rom_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("ch8"))
        {
            self.files.push(entry.path().to_owned().into());
        }
    }

    pub fn select(&mut self) -> Option<&OsString> {
        self.walk();

        if self.files.is_empty() {
            eprintln!("No ROM files found");
            return None;
        }

        let options: Vec<&str> = self
            .files
            .iter()
            .map(|p| std::path::Path::new(p).file_stem().unwrap())
            .filter_map(|s| s.to_str())
            .collect();

        let ans: Result<&str, InquireError> = Select::new("Select/Search: ", options).prompt();

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
            Err(e) => {
                eprintln!("ROM selection failed: {e}");
                None
            }
        }
    }
}
