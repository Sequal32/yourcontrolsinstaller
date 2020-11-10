use std::{collections::HashSet, fs, io::{self, Cursor}};

use bytes::Bytes;
use log::{info, warn};
use zip::ZipArchive;

use crate::util::{Error, Features, get_path_beginning, strip_path_beginning};

const COMMUNITY_PREFIX: &str = "PLACE IN COMMUNITY PACKAGES";
const OPTIONAL_PREFIX: &str = "OPTIONALS";

pub struct Installer {
    package_dir: String, 
    program_dir: String
}

impl Installer {
    pub fn new() -> Self { 
        Self { 
            package_dir: String::new(), 
            program_dir: String::new()
        } 
    }

    pub fn store_program_path(&self, path: &String) -> Result<(), io::Error> {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let (subkey, _) = hklm.create_subkey("Software\\YourControls")?;

        subkey.set_value("path", path)?;

        Ok(())
    }

    pub fn get_program_path_from_registry(&self) -> Result<String, io::Error> {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let subkey = hklm.open_subkey("Software\\YourControls")?;

        subkey.get_value("path")
    }

    pub fn set_package_dir(&mut self, package_dir: String) {
        self.package_dir = package_dir;
    }

    pub fn set_program_dir(&mut self, program_dir: String) {
        self.program_dir = program_dir;
    }

    fn get_path_for_file(&self, file_name: &str, options: &HashSet<String>) -> Option<String> {
        // Program files
        if !file_name.starts_with(COMMUNITY_PREFIX) {return Some(format!("{}\\{}", self.program_dir, file_name))}
        // Community files
        // Optionals/A32NXS/YourControls/SimObjects/Airplanes/Asobo_A320_NEO_STABLE/
        let reduced_path = strip_path_beginning(file_name).unwrap();
        // Core package files
        if !reduced_path.starts_with(OPTIONAL_PREFIX) {return Some(format!("{}\\{}", self.package_dir, reduced_path))}
        // A32NXS/YourControls/SimObjects/Airplanes/Asobo_A320_NEO_STABLE/
        let optional_reduced_path = strip_path_beginning(&reduced_path).unwrap();
            
        // Handle optional package files
        if let Some(optional_name) = get_path_beginning(&optional_reduced_path) {
            // If the user specified the following option...
            if options.contains(&optional_name) {
                // YourControls/SimObjects/Airplanes/Asobo_A320_NEO_STABLE/
                return Some(format!("{}\\{}", self.package_dir, strip_path_beginning(&optional_reduced_path).unwrap()));
            }
            
        }

        None
    }

    pub fn install(&self, contents: &mut ZipArchive<Cursor<Bytes>>, options: &Features) -> Result<(), Error> {
        // Convert features to unique path names
        let mut features = HashSet::new();
        for option in options {
            features.insert(option.path.clone());

            info!("Requested feature \"{}\"", option.name);
        }
        // Remove community package installation
        match fs::remove_dir_all(format!("{}\\YourControls", self.package_dir)) {
            Ok(_) => info!("Removed existing package installation {}", self.package_dir),
            Err(e) => {
                warn!("Could not remove package installation, Reason: {}", e);
            }
        };

        // Create any directories that do not exist
        fs::create_dir_all(self.package_dir.clone()).ok();
        fs::create_dir_all(self.program_dir.clone()).ok();

        // Overwrite installation 
        for i in 0..contents.len() {
            let mut file = contents.by_index(i).unwrap();

            // Determine whether community or program files
            let path = match self.get_path_for_file(file.name(), &features) {
                Some(path) => path,
                None => continue
            };

            if file.is_dir() {

                fs::create_dir(path).ok();

            } else {

                let mut file_handle = fs::File::create(path).unwrap();
                io::copy(&mut file, &mut file_handle).ok();

            }
        }

        match self.store_program_path(&self.program_dir.to_string()) {
            Ok(_) => info!("Wrote {} to registry.", self.program_dir),
            Err(e) => {
                warn!("Could not write to registry, Reason: {}", e);
            }
        }

        Ok(())
    }
}