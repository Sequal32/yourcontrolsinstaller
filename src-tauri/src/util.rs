use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum Error {
    EnviornmentError(std::env::VarError),
    IOError(std::io::Error),
    JsonError,
    JsonSerializationError(serde_json::Error),
    MissingQuote,
    ReleaseError,
    WebError(attohttpc::Error),
    ZipError(zip::result::ZipError),
}

impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EnviornmentError(e) => write!(f, "Could not read enviornmental variable. Reason: {}", e),
            Error::MissingQuote => write!(f, "Could not find quote in path while looking for packages."),
            Error::WebError(e) => write!(f, "Error sending web request. Reason: {}", e),
            Error::JsonError => write!(f, "Missing field in JSON."),
            Error::JsonSerializationError(e) => write!(f, "Error processing JSON data. Reason: {}", e),
            Error::ReleaseError => write!(f, "Could not fetch release data."),
            Error::ZipError(e) => write!(f, "Could not read release ZIP file. Reason: {}", e),
            Error::IOError(e) => write!(f, "An IO error occured. Error: {}", e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feature {
    pub name: String,
    pub path: String
}

pub type Features = Vec<Feature>;

pub fn strip_path_beginning(path: &str) -> Option<String> {
    if let Some(dir_slash) = path.find("/") {
        return Some(path[dir_slash + 1..].to_string())
    }
    return None
}

pub fn get_path_beginning(path: &str) -> Option<String> {
    if let Some(dir_slash) = path.find("/") {
        return Some(path[0..dir_slash].to_string())
    }
    return None
}