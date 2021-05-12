use std::fs;
use minidom::Element;
use colored::Colorize;
use std::path::PathBuf;

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_project() {
    let working_dir: PathBuf;

    match std::env::current_dir() {
        Ok(dir) => {working_dir = dir}
        Err(_) => {
            eprintln!("{}", "Failed to get current working directory. Exiting...".bright_red());
            std::process::exit(1);
        }
    }
    let mut manifest_location = working_dir;
    manifest_location.push("manifest.xml");

    // First step: compare available string.xml files to available languages in manifest.xml

    // Get languages from the manifest
    println!("{}", "Reading manifest...".bold());
    let languages = get_languages_from_manifest(fs::read_to_string(manifest_location).expect("No manifest.xml was found"));

    println!("{}", "Checking available string resources...".bold());
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
                    eprintln!("{}. {}", "No languages found in manifest.xml.".red(), "Have you declared any?".bold())
                }
                Some(element) => {
                    for child in element.children() {
                        println!("{} {}", "Detected a language:", child.text().bold().green());
                        languages.push(child.text());
                    }
                }
            }
        }
    }

    return languages;
}