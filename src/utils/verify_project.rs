use std::fs::ReadDir;
use std::fs;
use colored::Colorize;
use std::path::PathBuf;
use crate::utils::do_vectors_match::do_vectors_match;
use minidom::Element;

pub fn verify_project() {
    let mut manifest_location: PathBuf;
    match std::env::current_dir() {
        Ok(dir) => {
            manifest_location = dir;
            manifest_location.push("manifest.xml");
        }
        Err(_) => {
            eprintln!("{}", "Failed to get current working directory. Exiting...".bright_red());
            std::process::exit(1);
        }
    }
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
    println!("{}", "Reading manifest...".bold());
    let languages = get_languages_from_manifest(fs::read_to_string(manifest_location).expect("No manifest.xml was found"));

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
                        available_resources.push(entry);
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
    if do_vectors_match(languages, available_resources) {

    } else {
        eprintln!("{}", "Language resources don't match up. Please remove unused languages from manifest.xml.".bright_red().bold());
        std::process::exit(1);
    }
}

fn get_languages_from_manifest(manifest: String) -> Vec<String> {
    println!("{}", "Checking languages...".bold());
    let root: Element = manifest.parse().unwrap();
    let mut languages: Vec<String> = Vec::new();

    for children in root.children() {
        if children.is("application", "http://www.garmin.com/xml/connectiq") {
            let language_children = children.get_child("languages", "http://www.garmin.com/xml/connectiq");
            match language_children {
                None => {
                    eprintln!("{} {} {} {}?", "No languages found in manifest.xml.".red(), "Have you added an".bold(), "<iq:languages>".bold().green(), "block".bold());
                    std::process::exit(1);
                }
                Some(element) => {
                    for child in element.children() {
                        println!("{} {}", "Detected a language:", child.text().bold().green());
                        languages.push(child.text());
                    }
                    if languages.is_empty() {
                        eprintln!("{} {} {} {}?", "No languages found in manifest.xml.".red(), "Have you".bold(), "declared".bold().green(), "any".bold());
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    return languages;
}