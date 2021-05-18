use minidom::Element;
use colored::Colorize;
use crate::utils::config::Config;
use yaserde_derive::YaSerialize;
use yaserde::ser::to_string;

pub fn generate_ciq_manifest(toml_config: String) -> String {
    let parsed_config: Config = toml::from_str(&*toml_config).unwrap();

    let mut ciq_products: Vec<CIQProduct> = Vec::new();
    let mut ciq_permissions: Vec<CIQPermission> = Vec::new();
    let mut ciq_languages: Vec<String> = Vec::new();

    for entry in parsed_config.package_meta.devices {
        ciq_products.push(CIQProduct{ id: entry })
    }
    for entry in parsed_config.package_meta.permissions {
        ciq_permissions.push(CIQPermission{ id: entry })
    }
    for entry in parsed_config.package_meta.languages {
        ciq_languages.push(entry)
    }

    let manifest_struct = CIQManifest {
        xmlns: "http://www.garmin.com/xml/connectiq".parse().unwrap(),
        version: 3,
        application: CIQApplication {
            entry: parsed_config.package.main_class,
            app_type: parsed_config.package.app_type,
            id: parsed_config.package_meta.id,
            launcher_icon: parsed_config.package_meta.icon_resource,
            name: parsed_config.package.name,
            version: parsed_config.package_meta.version,
            min_sdk_version: parsed_config.package.min_sdk,
            products: CIQProducts {
                product: ciq_products
            },
            permissions: CIQPermissions {
                uses_permission: ciq_permissions
            },
            languages: CIQLanguages { language: ciq_languages }
        }
    };
    return to_string(&manifest_struct).unwrap()
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
                        for child in child.attrs() {
                            devices.push(child.1.to_string());
                        }
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
