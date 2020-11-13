use std::{collections::HashSet, fs, io::{self, Cursor}};
use log::{error, info, warn};
use zip::ZipArchive;

use crate::{sizegenerator::SizeGenerator, util::{Error, Features, get_path_beginning, strip_path_beginning}};

const COMMUNITY_PREFIX: &str = "PLACE IN COMMUNITY PACKAGES";
const OPTIONAL_PREFIX: &str = "OPTIONALS";

enum InstallLocation {
    Package,
    Program
}

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

    fn get_relative_path_for_file(&self, file_name: &str, options: &HashSet<String>) -> Option<(String, InstallLocation)> {
        // Program files
        if !file_name.starts_with(COMMUNITY_PREFIX) {return Some((file_name.to_string(), InstallLocation::Program))}
        // Community files
        // Optionals/A32NXS/YourControls/SimObjects/Airplanes/Asobo_A320_NEO_STABLE/
        let reduced_path = strip_path_beginning(file_name).unwrap();
        // Core package files
        if !reduced_path.starts_with(OPTIONAL_PREFIX) {return Some((reduced_path, InstallLocation::Package))}
        // A32NXS/YourControls/SimObjects/Airplanes/Asobo_A320_NEO_STABLE/
        let optional_reduced_path = strip_path_beginning(&reduced_path).unwrap();
            
        // Handle optional package files
        if let Some(optional_name) = get_path_beginning(&optional_reduced_path) {
            // If the user specified the following option...
            if options.contains(&optional_name) {
                // YourControls/SimObjects/Airplanes/Asobo_A320_NEO_STABLE/
                return Some((strip_path_beginning(&optional_reduced_path).unwrap(), InstallLocation::Package));
            }
            
        }

        None
    }

    pub fn remove_package(&self) -> Result<(), io::Error> {
        match fs::remove_dir_all(format!("{}\\YourControls", self.package_dir)) {
            Ok(_) => {
                info!("Removed existing package installation {}", self.package_dir);
                Ok(())
            },
            Err(e) => {
                warn!("Could not remove package installation, Reason: {}", e);
                Err(e)
            }
        }
    }

    pub fn install(&self, contents: &mut ZipArchive<Cursor<Vec<u8>>>, options: &Features) -> Result<(), Error> {
        // Convert features to unique path names
        let mut features = HashSet::new();
        for option in options {
            features.insert(option.path.clone());

            info!("Requested feature \"{}\"", option.name);
        }
        // Remove community package installation
        self.remove_package();

        // Create any directories that do not exist
        fs::create_dir_all(self.package_dir.clone()).ok();
        fs::create_dir_all(self.program_dir.clone()).ok();

        // Generate layout.json
        let mut generator = SizeGenerator::new();

        // Overwrite installation 
        for i in 0..contents.len() {
            let mut file = contents.by_index(i).unwrap();

            // Determine whether community or program files
            let (relative_path, install_location) = match self.get_relative_path_for_file(file.name(), &features) {
                Some(d) => d,
                None => continue
            };

            let full_path = match install_location {
                InstallLocation::Package => {
                    if file.is_file() {
                        // Add file to generator
                        match generator.add_file(strip_path_beginning(&relative_path).unwrap(), file.size(), file.last_modified().to_time().to_timespec().sec) {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Could not add {} to layout.json generator!", file.name());
                                return Err(Error::IOError(e))
                            }
                        };
                    }

                    format!("{}\\{}", self.package_dir, relative_path)
                }
                InstallLocation::Program => {
                    format!("{}\\{}", self.program_dir, relative_path)
                }
            };
            
            // Write dir
            info!("Writing {}", full_path);
            if file.is_dir() {

                fs::create_dir(full_path).ok();

            } else {
            // Write file
                let mut file_handle = match fs::File::create(full_path) {
                    Ok(file) => file,
                    Err(e) => return Err(Error::IOError(e))
                };

                io::copy(&mut file, &mut file_handle).ok();
            }
        }

        match self.store_program_path(&self.program_dir.to_string()) {
            Ok(_) => info!("Wrote {} to registry.", self.program_dir),
            Err(e) => {
                warn!("Could not write to registry, Reason: {}", e);
            }
        }

        // Write layout.json
        match generator.write_to_file(&format!("{}\\YourControls\\layout.json", self.package_dir)) {
            Ok(_) => {}
            Err(e) => {
                error!("Could not write layout.json!");
                return Err(e)
            }
        }

        Ok(())
    }
}