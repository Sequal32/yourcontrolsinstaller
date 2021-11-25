use chrono::offset::Utc;
use serde::Serialize;
use serde_json::json;
use std::{fs::File, path::Path};

use crate::util::Error;

const DATE_CONSTANT: i64 = 116444736000000000;

fn get_current_time() -> i64 {
    DATE_CONSTANT + Utc::now().timestamp() * 10000000
}

#[derive(Serialize)]
struct FileData {
    path: String,
    size: u64,
    date: i64,
}

pub struct SizeGenerator {
    file_data: Vec<FileData>,
}

impl SizeGenerator {
    pub fn new() -> Self {
        Self {
            file_data: Vec::new(),
        }
    }

    pub fn add_file(&mut self, relative_path: String, size: u64) -> std::io::Result<()> {
        self.file_data.push(FileData {
            path: relative_path,
            size,
            date: get_current_time(),
        });

        Ok(())
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let data = json!({
            "content": self.file_data
        });

        match File::create(path) {
            Ok(file) => match serde_json::to_writer_pretty(file, &data) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::JsonSerializationError(e)),
            },
            Err(e) => Err(Error::IOError(e)),
        }
    }
}
