#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod downloader;
mod finder;
mod installer;
mod sizegenerator;
mod util;

use crossbeam_channel::unbounded;
use downloader::{Downloader, ReleaseData};
use installer::Installer;
use log::{error, info};
use serde::Serialize;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::path::PathBuf;
use std::thread::{sleep, spawn};
use std::time::Duration;
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
    release_data: Option<ReleaseData>,
}

enum AppMessage {
    Browse(String),
    BrowseResult(Result<dialog::Response, ()>),
    Shutdown,
}

fn main() {
    // Setup logging
    if let Ok(file) = File::create("log.txt") {
        WriteLogger::init(LevelFilter::Info, Config::default(), file).ok();
    }

    let mut installer = Installer::new();
    let mut downloader = Downloader::new();

    // Fetch latest release data
    let release_data = match downloader.get_data() {
        Ok(data) => data.cloned(),
        Err(e) => {
            error!(
                "Could not fetch latest release data from GitHub. Reason: {}",
                e
            );
            None
        }
    };

    // Default install to appdata
    let default_install_path = match installer.get_program_path_from_registry() {
        Ok(path) => {
            info!("Found previous installation path.");
            path
        }
        Err(_) => match env::var("APPDATA") {
            Ok(path) => {
                info!("Using default installation path.");
                PathBuf::from(path)
            }
            Err(e) => {
                error!("Could not use any installation path. Reason: {}", e);
                PathBuf::from("YourControls")
            }
        },
    };

    info!("Installation path: {:?}", default_install_path);
    installer.set_program_dir(default_install_path.clone());
    //
    let default_package_path = match finder::FlightSimFinder::get_package_location() {
        Ok(path) => {
            info!("Found package location: {:?}", path);
            path
        }
        Err(e) => {
            error!("Could not find any installation path. Reason: {:?}", e);
            PathBuf::from("Community/YourControls")
        }
    };

    installer.set_package_dir(default_package_path.clone());
    //
    let feature_list = match downloader.get_features() {
        Ok(list) => {
            info!("Fetched {} features.", list.len());
            Some(list)
        }
        Err(e) => {
            error!("Could not get features list! Reason: {}", e);
            None
        }
    };

    // Handle interthread communication
    let (to_app_tx, to_app_rx) = unbounded::<AppMessage>();
    let (to_main_tx, to_main_rx) = unbounded::<AppMessage>();

    let to_main_tx2 = to_main_tx.clone();

    spawn(move || {
        tauri::AppBuilder::new()
            .invoke_handler(move |_webview, arg| {
                use cmd::Cmd::*;

                match serde_json::from_str(arg) {
                    Err(e) => Err(e.to_string()),
                    Ok(command) => {
                        match command {
                            Startup { callback, error } => {
                                let default_install_path = default_install_path.clone();
                                let default_package_path = default_package_path.clone();
                                let feature_list = feature_list.clone();
                                let release_data = release_data.clone();

                                tauri::execute_promise(
                                    _webview,
                                    move || {
                                        Ok(StartupResponse {
                                            feature_list: feature_list.clone(),
                                            package_directory: default_package_path
                                                .to_string_lossy()
                                                .into_owned(),
                                            program_directory: default_install_path
                                                .to_string_lossy()
                                                .into_owned(),
                                            release_data,
                                        })
                                    },
                                    callback,
                                    error,
                                );
                            }
                            // DIrectory browse
                            Browse {
                                browse_for,
                                callback,
                                error,
                            } => {
                                let open_path = match browse_for {
                                    cmd::BrowseFor::Program => installer.get_program_dir(),
                                    cmd::BrowseFor::Package => installer.get_package_dir(),
                                };

                                println!("{:?}", installer.get_package_dir());

                                to_main_tx
                                    .send(AppMessage::Browse(
                                        open_path.to_string_lossy().into_owned(),
                                    ))
                                    .ok();

                                let location = match to_app_rx.recv() {
                                    Ok(AppMessage::BrowseResult(Ok(dialog::Response::Okay(
                                        location,
                                    )))) => {
                                        let mut location_path = PathBuf::from(&location);

                                        match browse_for {
                                            cmd::BrowseFor::Program => {
                                                if !location_path.ends_with("YourControls") {
                                                    location_path.push("YourControls");
                                                }
                                                installer.set_program_dir(location_path);
                                            }
                                            cmd::BrowseFor::Package => {
                                                installer.set_package_dir(location_path);
                                            }
                                        };

                                        Ok(location)
                                    }
                                    _ => Err(CommandError {}.into()),
                                };

                                tauri::execute_promise(_webview, move || location, callback, error);
                            }

                            Install {
                                callback,
                                error,
                                features,
                                options,
                            } => {
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
                                    Ok(mut zip) => installer.install_sequence(
                                        &mut zip,
                                        &selected_features,
                                        options.contains("Desktop Shortcut"),
                                    ),
                                    Err(e) => Err(e),
                                };

                                // Return a result
                                tauri::execute_promise(
                                    _webview,
                                    || match result {
                                        Ok(_) => Ok(()),
                                        Err(e) => {
                                            error!("Installation failed! Reason: {}", e);
                                            Err(e.into())
                                        }
                                    },
                                    callback,
                                    error,
                                );
                            }

                            Uninstall { callback, error } => {
                                installer.uninstall();

                                tauri::execute_promise(
                                    _webview,
                                    || {
                                        // match result {
                                        //     Ok(_) => Ok(()),
                                        //     Err(e) => Err(e.into())
                                        // }
                                        Ok(())
                                    },
                                    callback,
                                    error,
                                );
                            }

                            Launch => {
                                match installer.launch() {
                                    Ok(_) => {}
                                    Err(e) => error!(
                                        "Could not automatically launch the program! Reason: {}",
                                        e
                                    ),
                                };

                                to_main_tx.send(AppMessage::Shutdown).ok();
                            }
                        }
                        Ok(())
                    }
                }
            })
            .build()
            .run();

        to_main_tx2.send(AppMessage::Shutdown).ok();
    });

    loop {
        match to_main_rx.recv() {
            Ok(AppMessage::Browse(path)) => {
                to_app_tx
                    .send(AppMessage::BrowseResult(
                        dialog::pick_folder(Some(path)).map_err(|_| ()),
                    ))
                    .ok();
            }
            Ok(AppMessage::Shutdown) => return,
            _ => {}
        }

        sleep(Duration::from_millis(100));
    }
}
