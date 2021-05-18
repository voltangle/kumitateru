use minidom::Element;
use colored::Colorize;
use crate::utils::config::Config;
use std::fs;
use yaserde_derive::YaSerialize;
use yaserde::ser::to_string;
use yaserde::de::from_str;

pub fn generate_ciq_manifest(toml_config: String) -> &'static str {
    let parsed_config: Config = toml::from_str(&*toml_config).unwrap();

    let manifest_struct = CIQManifest {
        xmlns: "http://www.garmin.com/xml/connectiq".parse().unwrap(),
        version: 3,
        application: CIQApplication {
            entry: "WheelLogCompanionApp".to_string(),
            app_type: "watch-app".to_string(),
            id: "a4db".to_string(),
            launcher_icon: "Drawables".to_string(),
            name: "AppName".to_string(),
            version: "2.0.0".to_string(),
            min_sdk_version: "1.2.0".to_string(),
            products: CIQProducts {
                product: vec![
                    CIQProduct { id: "id".parse().unwrap() },
                    CIQProduct { id: "id2".parse().unwrap() }
                ]
            },
            permissions: CIQPermissions {
                uses_permission: vec![
                    CIQPermission { id: "d".to_string() },
                    CIQPermission { id: "a".to_string() },
                    CIQPermission { id: "f".to_string() },
                    CIQPermission { id: "g".to_string() },
                    CIQPermission { id: "h".to_string() },
                ]
            },
            languages: CIQLanguages { language: vec![
                "ddd".to_string(),
                "ddd".to_string()
            ] }
        }
    };
    let manifest = to_string(&manifest_struct);
    println!("{}", manifest.unwrap());
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
        if children.is("Application", "http://www.garmin.com/xml/connectiq") {
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

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "manifest")]
struct CIQManifest {
    #[yaserde(attribute)]
    xmlns: String,
    #[yaserde(attribute)]
    version: i8,
    #[yaserde(child)]
    application: CIQApplication
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CIQApplication {
    #[yaserde(attribute)]
    entry: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "type")]
    app_type: String,
    #[yaserde(attribute)]
    id: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "launcherIcon")]
    launcher_icon: String,
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    version: String,
    #[yaserde(attribute)]
    #[yaserde(rename = "minSdkVersion")]
    min_sdk_version: String,
    #[yaserde(child)]
    products: CIQProducts,
    permissions: CIQPermissions,
    languages: CIQLanguages,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CIQProducts {
    #[yaserde(child)]
    product: Vec<CIQProduct>
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CIQProduct {
    #[yaserde(attribute)]
    id: String
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CIQPermissions {
    #[yaserde(child)]
    #[yaserde(rename = "uses-permission")]
    uses_permission: Vec<CIQPermission>
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CIQPermission {
    #[yaserde(attribute)]
    id: String
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CIQLanguages {
    #[yaserde(child)]
    language: Vec<String>
}
