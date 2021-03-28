use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fmt::Display, io, os::windows::ffi::OsStrExt, path::PathBuf};
use winapi::um::shellapi::ShellExecuteW;

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
            Error::EnviornmentError(e) => {
                write!(f, "Could not read enviornmental variable. Reason: {}", e)
            }
            Error::MissingQuote => write!(
                f,
                "Could not find quote in path while looking for packages."
            ),
            Error::WebError(e) => write!(f, "Error sending web request. Reason: {}", e),
            Error::JsonError => write!(f, "Missing field in JSON."),
            Error::JsonSerializationError(e) => {
                write!(f, "Error processing JSON data. Reason: {}", e)
            }
            Error::ReleaseError => write!(f, "Could not fetch release data."),
            Error::ZipError(e) => write!(f, "Could not read release ZIP file. Reason: {}", e),
            Error::IOError(e) => write!(f, "An IO error occured. Error: {}", e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feature {
    pub name: String,
    pub path: String,
    pub group: Option<String>,
}

pub type Features = Vec<Feature>;

pub fn strip_path_beginning(path: &str) -> Option<String> {
    if let Some(dir_slash) = path.find("/") {
        return Some(path[dir_slash + 1..].to_string());
    }
    return None;
}

pub fn get_path_beginning(path: &str) -> Option<String> {
    if let Some(dir_slash) = path.find("/") {
        return Some(path[0..dir_slash].to_string());
    }
    return None;
}

pub fn to_u16s<S: AsRef<OsStr>>(s: S) -> io::Result<Vec<u16>> {
    fn inner(s: &OsStr) -> io::Result<Vec<u16>> {
        let mut maybe_result: Vec<u16> = s.encode_wide().collect();
        if maybe_result.iter().any(|&u| u == 0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "strings passed to WinAPI cannot contain NULs",
            ));
        }
        maybe_result.push(0);
        Ok(maybe_result)
    }
    inner(s.as_ref())
}

pub fn launch_program(path: PathBuf, arg: Option<&str>) {
    let open_bytes = to_u16s("open").unwrap();
    let path_bytes = to_u16s(path).unwrap();

    let arg_bytes = arg.map(|x| to_u16s(OsStr::new(x)).unwrap());

    unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            open_bytes.as_ptr(),
            path_bytes.as_ptr(),
            arg_bytes.map_or_else(|| std::ptr::null(), |x| x.as_ptr()),
            std::ptr::null(),
            winapi::ctypes::c_int::from(5),
        );
    }
}
