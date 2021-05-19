use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfig {
    pub package: AppConfigPackage,
    pub package_meta: AppConfigPackageMeta,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfigPackage {
    pub name: String,
    pub main_class: String,
    pub app_type: String,
    pub min_sdk: String,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfigPackageMeta {
    pub id: String,
    pub version: String,
    pub icon_resource: String,
    pub devices: Vec<String>,
    pub permissions: Vec<String>,
    pub languages: Vec<String>,
}

pub fn parse_config(config: String) -> AppConfig {
    let parsed_config: AppConfig = toml::from_str(&*config).unwrap();
    return parsed_config
}
