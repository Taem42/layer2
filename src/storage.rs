use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::consensus::StorageAdapter;

pub struct Storage {
    path: PathBuf,
}

impl StorageAdapter for Storage {
    fn write(&self, height: u64, data: String) {
        self.write(height, data)
    }
}

impl Storage {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        if !path.as_ref().exists() {
            std::fs::create_dir_all(&path).expect("Failed to create wal directory");
        }

        Storage {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn write(&self, height: u64, data: String) {
        let mut dir = self.path.clone();
        dir.push(height.to_string());
        dir.set_extension("txt");

        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(dir)
            .unwrap();

        file.write_all(&data.into_bytes()).unwrap();
    }
}
