use yaserde_derive::YaSerialize;
use yaserde::ser::to_string;
use crate::ser_de::config::app_config::AppConfig;
use anyhow::{Result, Context};

pub fn generate_ciq_manifest(config: AppConfig) -> Result<String> {
    let mut ciq_products: Vec<CIQProduct> = Vec::new();
    let mut ciq_permissions: Vec<CIQPermission> = Vec::new();
    let mut ciq_languages: Vec<String> = Vec::new();
    let mut ciq_dependencies: Vec<CIQDependency> = Vec::new();

    for entry in config.package_meta.devices {
        ciq_products.push(CIQProduct{ id: entry })
    }
    for entry in config.package_meta.permissions {
        ciq_permissions.push(CIQPermission{ id: entry })
    }
    for entry in config.package_meta.languages {
        ciq_languages.push(entry)
    }
    for (entry, value) in config.dependencies {
        ciq_dependencies.push(CIQDependency {
            name: entry,
            version: value[0].as_str().with_context(|| format!("Unable to convert value {} to string", value))?.to_string()
        })
    }

    let manifest_struct = CIQManifest {
        xmlns: "http://www.garmin.com/xml/connectiq".parse()?,
        version: 3,
        application: CIQApplication {
            entry: config.package.main_class,
            app_type: config.package.app_type,
            id: config.package_meta.id,
            launcher_icon: config.package.icon_resource,
            name: config.package.name_res,
            version: config.package_meta.version,
            min_sdk_version: config.package.min_sdk,
            products: CIQProducts {
                product: ciq_products
            },
            permissions: CIQPermissions {
                uses_permission: ciq_permissions
            },
            languages: CIQLanguages { language: ciq_languages },
            barrels: CIQDependencies {
                depends: ciq_dependencies
            }
        }
    };
    let mut serialized_manifest = to_string(&manifest_struct)?;

    // And then a series of string replaces for adding iq namespace
    serialized_manifest = serialized_manifest
        .replace("<manifest", "<iq:manifest")
        .replace("</manifest", "</iq:manifest")
        .replace("xmlns", "xmlns:iq")
        .replace("<application", "<iq:application")
        .replace("</application", "</iq:application")
        .replace("<products", "<iq:products")
        .replace("</products", "</iq:products")
        .replace("<product", "<iq:product")
        .replace("<permissions", "<iq:permissions")
        .replace("</permissions", "</iq:permissions")
        .replace("<uses-permission", "<iq:uses-permission")
        .replace("<languages", "<iq:languages")
        .replace("</languages", "</iq:languages")
        .replace("<language", "<iq:language")
        .replace("</language", "</iq:language")
        .replace("<barrels", "<iq:barrels")
        .replace("</barrels", "</iq:barrels")
        .replace("<depends", "<iq:depends");
    return Ok(serialized_manifest)
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
    barrels: CIQDependencies,
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

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CIQDependencies {
    depends: Vec<CIQDependency>
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
struct CIQDependency {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    version: String
}