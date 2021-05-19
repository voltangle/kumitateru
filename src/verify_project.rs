use std::fs::ReadDir;
use std::fs;
use colored::Colorize;
use std::path::PathBuf;
use crate::utils::do_vectors_match::do_vectors_match;
use crate::utils::manifest_utils::{get_devices_from_manifest, get_languages_from_manifest};
use std::borrow::Cow;
use crate::utils::config::parse_config;

pub fn verify_project() {
    let mut resources_location: PathBuf;
    match std::env::current_dir() {
        Ok(dir) => {
            resources_location = dir;
            resources_location.push("resources");
        }
        Err(_) => {
            eprintln!("{}", "Failed to get current working directory. Exiting...".bright_red());
            std::process::exit(1);
        }
    }
    let mut resources_strings_location: PathBuf;
    match std::env::current_dir() {
        Ok(dir) => {
            resources_strings_location = dir;
            resources_strings_location.push("resources");
            resources_strings_location.push("strings");
        }
        Err(_) => {
            eprintln!("{}", "Failed to get current working directory. Exiting...".bright_red());
            std::process::exit(1);
        }
    }


    // First step: compare available string.xml files to available languages in manifest.xml

    // Get languages from the manifest
    println!("{}", "Reading config...".bold());
    let config_string = fs::read_to_string(PathBuf::from("kumitateru.toml")).expect("No kumitateru.toml was found");
    let parsed_config = parse_config(config_string.clone());

    let string_resource_directories: ReadDir;
    match fs::read_dir(resources_strings_location) {
        Ok(dir) => {
            string_resource_directories = dir;
        }
        Err(_) => {
            eprintln!("{}", "Failed to get strings in string resources. Exiting...".bright_red());
            std::process::exit(1);
        }
    }

    let mut available_resources: Vec<String> = Vec::new();

    for entry in string_resource_directories {
        match entry {
            Ok(entry) => {
                match entry.file_name().into_string() {
                    Ok(entry) => {
                        if entry != ".DS_Store" {
                            available_resources.push(entry);
                        }
                    }
                    Err(_) => {
                        eprintln!("{}", "Something had gone wrong while reading files. Exiting...".bright_red());
                        std::process::exit(1);
                    }
                }
            }
            Err(_) => {
                eprintln!("{}", "Something had gone wrong while reading files. Exiting...".bright_red());
                std::process::exit(1);
            }
        }
    }
    if do_vectors_match(parsed_config.clone().package_meta.languages, available_resources) {} else {
        eprintln!("{}", "Language resources don't match up. Please remove unused languages from manifest.xml.".bright_red().bold());
        std::process::exit(1);
    }
    // Next step: check for device-specific resources, that reference not-supported devices(not declared in manifest)
    // Check device-specific resources
    for entry in fs::read_dir("resources") {
        for entry in entry {
            let entry = entry.unwrap();

            // This is needed to skip strings, because they contain folders with translated strings, instead of device-specific ones.
            if entry.path() == PathBuf::from("resources/strings") {
                continue; // Continues to the next index, skipping next code
            }
            let mut resources: Vec<String> = Vec::new();

            for entry in fs::read_dir(entry.path()) {
                for entry in entry {
                    let entry = entry.unwrap();
                    if entry.file_type().unwrap().is_dir() {
                        let entry_string = entry.file_name().into_string().unwrap();
                        resources.push(basename(&*entry_string, '/').to_string());
                    }
                }
            }
            match_device_resources(parsed_config.clone().package_meta.devices, resources.clone())
        }
    }

    println!("{}", "Successfully verified project structure!".bold().green())
}

fn basename(path: &str, sep: char) -> Cow<str> {
    let mut pieces = path.rsplit(sep);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

fn match_device_resources(manifest: Vec<String>, res: Vec<String>) {
    if res.len() > 0 {
        for res in res {
            if !manifest.contains(&res) {
                eprintln!("{}", "Detected device-specific resource declarations for devices that \
                are not declared as supported in manifest. Please, \
                remove these resources, or add missing device in manifest.".red().bold());
                std::process::exit(1);
            }
        }
    }
}