use serde::Deserialize;
use toml::value::Table;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfig {
    pub package: AppConfigPackage,
    pub package_meta: AppConfigPackageMeta,
    pub build: AppConfigBuild,
    pub dependencies: Table,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct AppConfigPackage {
    pub icon_resource: String,
    /// Name resource.
    pub name_res: String,
    /// Main class, which will be called on start.
    pub main_class: String,
    /// App type.
    pub app_type: String,
    /// Minimum SDK, on which the app/library will run.
    pub min_sdk: String,
    /// SDK which will be used to compile the project.
    pub target_sdk: String,
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
    pub code_analysis_on_build: bool,
    pub type_check_level: i8,
    pub compiler_args: String,
}
