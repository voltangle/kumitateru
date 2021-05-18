use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub package: ConfigPackage,
    pub package_meta: ConfigPackageMeta,
}

#[derive(Deserialize)]
pub struct ConfigPackage {
    pub name: String,
    pub main_class: String,
    pub app_type: String,
    pub min_sdk: String,
}

#[derive(Deserialize)]
pub struct ConfigPackageMeta {
    pub id: String,
    pub version: String,
    pub icon: String,
    pub devices: Box<[String]>,
    pub permissions: Box<[String]>,
    pub languages: Box<[String]>,
}
