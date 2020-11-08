use std::{fs, io::{self, Cursor}};

use bytes::Bytes;
use zip::ZipArchive;

use crate::util::{Features, strip_path_beginning};

const COMMUNITY_PREFIX: &str = "PLACE IN COMMUNITY PACKAGES";

pub struct Installer {
}

impl Installer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn store_program_path(&self, path: &String) -> Result<(), io::Error> {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let (subkey, _) = hklm.create_subkey("Software\\YourControls")?;

        subkey.set_value("path", path)?;

        Ok(())
    }

    pub fn get_program_path(&self) -> Result<String, io::Error> {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let subkey = hklm.open_subkey("Software\\YourControls")?;

        subkey.get_value("path")
    }

    pub fn install(&self, contents: &mut ZipArchive<Cursor<Bytes>>, package_dir: &str, program_dir: &str, options: Features) -> Result<(), io::Error> {
        // Remove community package installation
        fs::remove_dir_all(format!("{}\\YourControls", package_dir)).ok();
        // Overwrite installation 
        for i in 0..contents.len() {
            let mut file = contents.by_index(i).unwrap();

            // Determine whether community or program files
            let path = if file.name().starts_with(COMMUNITY_PREFIX) {
                format!("{}\\{}", package_dir, strip_path_beginning(file.name()).unwrap())
            } else {
                format!("{}\\{}", program_dir, file.name())
            };

            if file.is_dir() {

                fs::create_dir(path).ok();

            } else {

                let mut file_handle = fs::File::create(path).unwrap();
                io::copy(&mut file, &mut file_handle).ok();

            }
        }

        self.store_program_path(&program_dir.to_string());

        Ok(())
    }
}