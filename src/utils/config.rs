use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfig {
    pub package: AppConfigPackage,
    pub package_meta: AppConfigPackageMeta,
    pub build: AppConfigBuild,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfigPackage {
    pub icon_resource: String,
    pub name_res: String,
    pub main_class: String,
    pub app_type: String,
    pub min_sdk: String,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfigPackageMeta {
    pub name: String,
    pub id: String,
    pub version: String,
    pub devices: Vec<String>,
    pub permissions: Vec<String>,
    pub languages: Vec<String>,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfigBuild {
    pub signing_key: String,
    pub enable_code_analysing: bool,
    pub connect_iq_version: String,
    pub compiler_args: String,
}

pub fn parse_config(config: String) -> AppConfig {
    return toml::from_str::<AppConfig>(&*config).unwrap();
}
