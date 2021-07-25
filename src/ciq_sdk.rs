use std::{env, fs, process};
use std::path::PathBuf;

pub struct CIQSdk {}

impl CIQSdk {
    pub fn bin_location(sdk_version: &str) -> PathBuf {
        return match env::consts::OS {
            "macos" => {
                // Searching for SDKs
                let mut sdk_versions: Vec<String> = Vec::new();
                let mut sdk_paths: Vec<PathBuf> = Vec::new();
                let home_dir = home::home_dir().unwrap();
                let sdk_dir_path = PathBuf::from(format!("{}{}", home_dir.to_str().unwrap(), "/Library/Application Support/Garmin/ConnectIQ/Sdks/"));
                for path in fs::read_dir(sdk_dir_path).unwrap() {
                    let path = path.unwrap();
                    sdk_paths.push(path.path());
                    let path = path.file_name().to_str().unwrap().to_string();
                    let sdk_version = &path[18..23];
                    sdk_versions.push(sdk_version.to_string());
                }
                if sdk_versions.contains(&sdk_version.to_string()) {
                    let pos = sdk_versions.clone().iter().position(|r| r == sdk_version).unwrap();
                    let mut path = (&*sdk_paths[pos]).to_path_buf();
                    path.push("bin");
                    path
                } else {
                    eprintln!("Sorry, could not find any SDKs. Please download any!");
                    process::exit(24); // Error code 24 signifies that kumitateru was unable to find any SDKs because the CIQ folder did not exist or was empty.
                }
            }
            "windows" => {
                // Searching for SDKs
                let mut sdk_versions: Vec<String> = Vec::new();
                let mut sdk_paths: Vec<PathBuf> = Vec::new();
                let home_dir = home::home_dir().unwrap();
                let sdk_dir_path = PathBuf::from(format!("{}{}", home_dir.to_str().unwrap(), "\\AppData\\Roaming\\Garmin\\ConnectIQ\\Sdks"));
                for path in fs::read_dir(sdk_dir_path).unwrap() {
                    let path = path.unwrap();
                    sdk_paths.push(path.path());
                    let path = path.file_name().to_str().unwrap().to_string();
                    let sdk_version = &path[18..23];
                    sdk_versions.push(sdk_version.to_string());
                }
                if sdk_versions.contains(&sdk_version.to_string()) {
                    let pos = sdk_versions.clone().iter().position(|r| r == sdk_version).unwrap();
                    let mut path = (&*sdk_paths[pos]).to_path_buf();
                    path.push("bin");
                    path
                } else {
                    eprintln!("Sorry, could not find any SDKs. Please download any!");
                    process::exit(24); // Error code 24 signifies that kumitateru was unable to find any SDKs because the CIQ folder did not exist or was empty.
                }
            }
            &_ => {
                eprintln!("Sorry, unsupported OS. Please, run this binary only on supported OS'es(macOS and Windows)");
                process::exit(25); // Error code 25 signifies that kumitateru was unable to find any SDKs because the system is unsupported.
            }
        }
    }
}
