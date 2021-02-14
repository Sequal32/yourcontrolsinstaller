use std::{ffi::OsStr, path::Path};
#[cfg(windows)]
use std::{collections::HashSet, fs, io::{self, Cursor}, path::{PathBuf}};
use log::{error, info, warn};
use zip::{ZipArchive, read::ZipFile};

use crate::{sizegenerator::SizeGenerator, util::{Error, Features, get_path_beginning, launch_program, strip_path_beginning}};

const COMMUNITY_PREFIX: &str = "community";
const OPTIONAL_PREFIX: &str = "optionals";

const EXE_NAME: &str = "YourControls.exe";

enum InstallLocation {
    Package,
    Program
}

fn add_to_generator(generator: &mut SizeGenerator, relative_path: &str, file: &ZipFile) -> Result<(), Error> {
    if file.name().contains("layout.json") || file.name().contains("manifest.json") {return Ok(())}
    // Add file to generator
    match generator.add_file(strip_path_beginning(relative_path).unwrap(), file.size()) {
        Ok(_) => {}
        Err(e) => {
            error!("Could not add {} to layout.json generator!", file.name());
            return Err(Error::IOError(e))
        }
    };

    Ok(())
}

pub struct Installer {
    package_dir: PathBuf, 
    program_dir: PathBuf
}

impl Installer {
    pub fn new() -> Self { 
        Self { 
            package_dir: PathBuf::new(), 
            program_dir: PathBuf::new()
        } 
    }

    pub fn store_program_path(&self, path: &str) -> Result<(), io::Error> {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let (subkey, _) = hklm.create_subkey("Software\\YourControls")?;

        subkey.set_value("path", &path)?;

        Ok(())
    }

    pub fn get_program_path_from_registry(&self) -> Result<PathBuf, io::Error> {
        let hklm = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let subkey = hklm.open_subkey("Software\\YourControls")?;

        subkey.get_value("path").map(|x: String| PathBuf::from(x))
    }

    pub fn set_package_dir(&mut self, package_dir: PathBuf) {
        self.package_dir = package_dir;
    }

    pub fn set_program_dir(&mut self, program_dir: PathBuf) {
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
        let package_dir = self.package_dir.join("YourControls");

        match fs::remove_dir_all(package_dir) {
            Ok(_) => {
                info!("Removed existing package installation {:?}", self.package_dir);
                Ok(())
            },
            Err(e) => {
                warn!("Could not remove package installation, Reason: {}", e);
                Err(e)
            }
        }
    }

    pub fn remove_exe(&self) -> Result<(), io::Error> {
        let path = self.get_exe_path();

        if path.exists() {
            return match fs::remove_dir_all(&self.program_dir) {
                Ok(_) => {
                    info!("Removed exe folder contents at {:?}", self.program_dir);
                    Ok(())
                },
                Err(e) => {
                    warn!("Could not remove exe installation, Reason: {}", e);
                    Err(e)
                }
            }
        }

        Ok(())
    }

    pub fn uninstall(&self) {
        self.remove_package().ok();
        self.remove_exe().ok();
    }

    pub fn get_exe_path(&self) -> PathBuf {
        self.program_dir.join(EXE_NAME)
    }

    pub fn get_program_dir(&self) -> &PathBuf {
        &self.program_dir
    }

    pub fn get_package_dir(&self) -> &PathBuf {
        &self.package_dir
    }

    pub fn install(&self, contents: &mut ZipArchive<Cursor<Vec<u8>>>, features: &Features, create_shortcut: bool) -> Result<(), Error> {
        // Convert features to unique path names
        let mut feature_paths = HashSet::new();
        for option in features {
            feature_paths.insert(option.path.clone());

            info!("Requested feature \"{}\"", option.name);
        }
        // Remove community package installation
        self.remove_package().ok();

        // Create any directories that do not exist
        fs::create_dir_all(self.package_dir.clone()).ok();
        fs::create_dir_all(self.program_dir.clone()).ok();

        // Generate layout.json
        let mut generator = SizeGenerator::new();

        // Overwrite installation 
        for i in 0..contents.len() {
            let mut content = contents.by_index(i).unwrap();

            // Determine whether community or program files
            let (relative_path, install_location) = match self.get_relative_path_for_file(content.name(), &feature_paths) {
                Some(d) => d,
                None => continue
            };

            let full_path = match install_location {
                InstallLocation::Package => {
                    if content.is_file() {
                        add_to_generator(&mut generator, &relative_path, &content)?;
                    }

                    self.package_dir.join(relative_path)
                }
                InstallLocation::Program => {
                    self.program_dir.join(relative_path)
                }
            };
            
            // Write dir
            info!("Writing {:?}", full_path);
            if content.is_file() {
                info!("Creating dir(s) {:?}", full_path.parent());
                fs::create_dir_all(full_path.parent().unwrap()).ok();
            // Write file
                let mut file_handle = match fs::File::create(full_path) {
                    Ok(file) => file,
                    Err(e) => return Err(Error::IOError(e))
                };

                io::copy(&mut content, &mut file_handle).ok();
            }
        }

        match self.store_program_path(&self.program_dir.to_str().unwrap_or("")) {
            Ok(_) => info!("Wrote {:?} to registry.", self.program_dir),
            Err(e) => {
                warn!("Could not write to registry, Reason: {}", e);
            }
        }

        // Write layout.json
        match generator.write_to_file(self.package_dir.join("YourControls/layout.json")) {
            Ok(_) => {}
            Err(e) => {
                error!("Could not write layout.json!");
                return Err(e)
            }
        }

        // Generate shortcut
        if create_shortcut {
            // Get relative path
            let mut path = std::env::current_exe().unwrap();
            // Don't need exe path
            path.pop();
            // Push further path
            path.push("shortcutcreator.exe");

            launch_program(path, None);
        }

        Ok(())
    }

    pub fn launch(&self) -> Result<(), io::Error> {
        let mut process = std::process::Command::new(self.get_exe_path());
        process
            .current_dir(self.get_program_dir())
            .stderr(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stdin(std::process::Stdio::null());

        match process.spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}