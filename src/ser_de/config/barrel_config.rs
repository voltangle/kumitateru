use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct BarrelConfig {
    pub package: BarrelConfigPackage,
    pub package_meta: BarrelConfigPackageMeta,
    pub build: BarrelConfigBuild,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct BarrelConfigPackage {
    pub icon_resource: String,
    pub name_res: String,
    pub main_class: String,
    pub app_type: String,
    pub min_sdk: String,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct BarrelConfigPackageMeta {
    pub name: String,
    pub id: String,
    pub version: String,
    pub devices: Vec<String>,
    pub permissions: Vec<String>,
    pub annotations: Vec<String>,
    pub languages: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct BarrelConfigBuild {
    pub signing_key: String,
    pub enable_code_analysis_on_build: bool,
    pub connect_iq_version: String,
    pub compiler_args: String,
}
