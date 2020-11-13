#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod downloader;
mod installer;
mod finder;
mod sizegenerator;
mod util;

use std::{env, fs::File};
use std::fmt::Display;
use downloader::{Downloader, ReleaseData};
use log::{error, info};
use simplelog::{Config, LevelFilter, WriteLogger};
use serde::Serialize;
use tauri::api::dialog;
use util::Feature;

#[derive(Debug)]
struct CommandError {}
impl std::error::Error for CommandError {}
impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "")
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StartupResponse {
    feature_list: Option<Vec<Feature>>,
    package_directory: String,
    program_directory: String,
    release_data: Option<ReleaseData>
}

fn main() {
    // Setup logging
    if let Ok(file) = File::create("log.txt") {
        WriteLogger::init(LevelFilter::Info, Config::default(), file).ok();
    }

    let mut installer =  installer::Installer::new();
    let mut downloader = Downloader::new();

    // Fetch latest release data
    let release_data = match downloader.get_data() {
        Ok(data) => data.cloned(),
        Err(e) => {
            error!("Could not fetch latest release data from GitHub. Reason: {}", e);
            None
        }
    };

    // Default install to appdata
    let default_install_path = match installer.get_program_path_from_registry() {
        Ok(path) => {
            info!("Found previous installation path.");
            path
        },
        Err(_) => match env::var("APPDATA") {
            Ok(path) => {
                info!("Using default installation path.");
                path + "\\YourControls"
            },
            Err(e) => {
                error!("Could not use any installation path. Reason: {}", e);
                "YourControls".to_string()
            }
        }
    };

    info!("Installation path: {}", default_install_path);
    installer.set_program_dir(default_install_path.clone());
    //
    let default_package_path = match finder::FlightSimFinder::get_package_location() {
        Ok(path) => {
            info!("Found package location: {}", path);

            path
        },
        Err(e) => {
            error!("Could not find any installation path. Reason: {}", e);
            "Community\\YourControls".to_string()
        }
    };

    installer.set_package_dir(default_package_path.clone());
    //
    let feature_list = match downloader.get_features() {
        Ok(list) => {
            info!("Fetched {} features.", list.len());
            Some(list)
        },
        Err(e) => {
            error!("Could not get features list! Reason: {}", e);
            None
        }
    };
    

    tauri::AppBuilder::new()
        .invoke_handler(move |_webview, arg| {
        use cmd::Cmd::*;
        
        match serde_json::from_str(arg) {
            Err(e) => {
                Err(e.to_string())
            }
            Ok(command) => {
                match command {
                    Startup {callback, error} => {
                        let default_install_path = default_install_path.clone();
                        let default_package_path = default_package_path.clone();
                        let feature_list = feature_list.clone();
                        let release_data = release_data.clone();

                        tauri::execute_promise(_webview, move || {

                            Ok(StartupResponse {
                                feature_list: feature_list.clone(),
                                package_directory: default_package_path,
                                program_directory: default_install_path,
                                release_data
                            })


                        }, callback, error);

                    }
                    // DIrectory browse
                    Browse {browse_for, callback, error} => {
                        let location = match dialog::pick_folder(Option::<String>::None) {
                            Ok(dialog::Response::Okay(mut location)) => {
                                
                                match browse_for {
                                    cmd::BrowseFor::Program => {
                                        location += "\\YourControls";
                                        installer.set_program_dir(location.clone())
                                    }
                                    cmd::BrowseFor::Package => {
                                        installer.set_package_dir(location.clone())
                                    }
                                };

                                Ok(location)

                            },
                            _ => Err(CommandError {}.into())
                        };
                        

                        tauri::execute_promise(_webview, move || {
                            location
                        }, callback, error);
                        
                    }

                    Install {callback, error, features} => {
                        let mut selected_features = Vec::new();

                        // Match list of possible features with selected features
                        if let Some(possible_features) = feature_list.as_ref() {
                            for feature in possible_features {
                                if features.contains(&feature.name) {
                                    selected_features.push(feature.clone())
                                }
                            }
                        }
                        // Download and install
                        let result = match downloader.download_release() {
                            Ok(mut zip) => installer.install(&mut zip, &selected_features),
                            Err(e) => Err(e)
                        };

                        // Return a result
                        tauri::execute_promise(_webview, || {
                            match result {
                                Ok(_) => Ok(()),
                                Err(e) => {
                                    error!("Installation failed! Reason: {}", e);
                                    Err(e.into())
                                }
                            }
                        }, callback, error);
                        
                    }

                    Uninstall {callback, error} => {
                        let result = installer.remove_package();

                        tauri::execute_promise(_webview, || {
                            match result {
                                Ok(_) => Ok(()),
                                Err(e) => Err(e.into())
                            }
                        }, callback, error);
                    }
                }
                Ok(())
            }
        }
        })
        .build()
        .run();
}
