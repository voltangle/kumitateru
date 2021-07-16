use yaserde_derive::YaSerialize;
use yaserde::ser::to_string;
use crate::ser_de::config::app_config::AppConfig;

pub fn generate_ciq_manifest(toml_config: String) -> String {
    let parsed_config: AppConfig = toml::from_str(&*toml_config).unwrap();

    let mut ciq_products: Vec<CIQProduct> = Vec::new();
    let mut ciq_permissions: Vec<CIQPermission> = Vec::new();
    let mut ciq_languages: Vec<String> = Vec::new();
    let mut ciq_dependencies: Vec<CIQDependency> = Vec::new();

    for entry in parsed_config.package_meta.devices {
        ciq_products.push(CIQProduct{ id: entry })
    }
    for entry in parsed_config.package_meta.permissions {
        ciq_permissions.push(CIQPermission{ id: entry })
    }
    for entry in parsed_config.package_meta.languages {
        ciq_languages.push(entry)
    }
    for (entry, value) in parsed_config.dependencies {
        ciq_dependencies.push(CIQDependency {
            name: entry,
            version: value.to_string()
        })
    }

    let manifest_struct = CIQManifest {
        xmlns: "http://www.garmin.com/xml/connectiq".parse().unwrap(),
        version: 3,
        application: CIQApplication {
            entry: parsed_config.package.main_class,
            app_type: parsed_config.package.app_type,
            id: parsed_config.package_meta.id,
            launcher_icon: parsed_config.package.icon_resource,
            name: parsed_config.package.name_res,
            version: parsed_config.package_meta.version,
            min_sdk_version: parsed_config.package.min_sdk,
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
    let mut serialized_manifest = to_string(&manifest_struct).unwrap();

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
    return serialized_manifest
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
    name: String,
    version: String
}