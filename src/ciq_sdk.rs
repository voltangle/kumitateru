use std::{env, fs};
use std::path::PathBuf;

pub struct CIQSdk {}

impl CIQSdk {
    pub fn bin_location(sdk_version: &str) -> &str {
        return match env::consts::OS {
            "linux" => {
                ""
            }
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
                    println!("Detected SDK: version {}", sdk_version.clone());
                    sdk_versions.push(sdk_version.to_string());
                }
                if sdk_versions.contains(&sdk_version.to_string()) {
                    let pos = sdk_versions.clone().iter().position(|r| r == sdk_version).unwrap();
                    ""
                } else {
                    ""
                }
                // "~/Library/Application Support/Garmin/ConnectIQ/Sdks/connectiq-sdk-mac-4.0.4-2021-07-01-9df386fcd/bin"

            }
            "windows" => {
                ""
            }
            &_ => {
                eprintln!("Sorry, unknown OS. Please, run this binary only on supported OS'es(macOS, Linux, Windows)");
                "badsys"
            }
        }
    }
}
