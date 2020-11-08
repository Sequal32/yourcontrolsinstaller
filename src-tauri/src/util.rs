use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    EnviornmentError(std::env::VarError),
    FileError(std::io::Error),
    MissingQuote,
    WebError(reqwest::Error),
    JsonError,
    ReleaseError,
    ZipError(zip::result::ZipError)
}

#[derive(Debug, Deserialize)]
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