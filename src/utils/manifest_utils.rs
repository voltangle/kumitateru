use minidom::Element;
use colored::Colorize;
use crate::utils::config::Config;

pub fn generate_manifest(toml_config: String) -> &'static str {
    let parsed_config: Config = toml::from_str(&*toml_config).unwrap();
    return ""
}

pub fn get_languages_from_manifest(manifest: String) -> Vec<String> {
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

pub fn get_devices_from_manifest(manifest: String) -> Vec<String> {
    let root: Element = manifest.parse().unwrap();
    let mut devices: Vec<String> = Vec::new();

    for children in root.children() {
        if children.is("application", "http://www.garmin.com/xml/connectiq") {
            let products_children = children.get_child("products", "http://www.garmin.com/xml/connectiq");
            match products_children {
                None => {
                    eprintln!("{} {} {} {}?", "No products found in manifest.xml.".red(), "Have you added an".bold(), "<iq:products>".bold().green(), "block".bold());
                    std::process::exit(1);
                }
                Some(element) => {
                    for child in element.children() {
                        devices.push(child.text());
                    }
                    if devices.is_empty() {
                        eprintln!("{} {} {} {}?", "No products found in manifest.xml.".red(), "Have you".bold(), "declared".bold().green(), "any".bold());
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    return devices;
}