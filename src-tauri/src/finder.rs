use std::{io::BufRead, env, fs::File, io::BufReader};

use crate::util::Error;

const STEAM_PATH: &str = "\\Microsoft Flight Simulator\\UserCfg.opt";
const MICROSOFT_PATH: &str = "\\Packages\\Microsoft.FlightSimulator_8wekyb3d8bbwe\\LocalCache\\UserCfg.opt";

pub struct FlightSimFinder {

}

impl FlightSimFinder {
    fn get_config_file_handle(env_var: &str, post_path: &str) -> Result<File, Error> {
        match env::var(env_var) {
            Ok(path) => match File::open(format!("{}{}", path, post_path)) {
                Ok(f) => return Ok(f),
                Err(e) => return Err(Error::IOError(e))
            }
            Err(e) => return Err(Error::EnviornmentError(e))
        }
    }
    
    fn get_config_file() -> Result<File, Error> {
        Ok(
            match Self::get_config_file_handle("APPDATA", STEAM_PATH) {
                Ok(f) => f,
                Err(_) => Self::get_config_file_handle("LOCALAPPDATA", MICROSOFT_PATH)?
            }
        )
    }

    pub fn get_package_location() -> Result<String, Error> {
        let file = Self::get_config_file()?;
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            if let Ok(line) = line {
            
                if line.starts_with("InstalledPackagesPath") {
                    
                    let first_quote = match line.find("\"") {
                        Some(index) => index,
                        None => return Err(Error::MissingQuote)
                    };
                    

                    let closing_quote = match line[first_quote + 1..].find("\"") {
                        Some(index) => index + first_quote,
                        None => return Err(Error::MissingQuote)
                    };

                    return Ok(line[first_quote + 1..closing_quote + 1].to_string() + "\\Community")

                }

            }
        }

        return Err(Error::MissingQuote)
    }
}