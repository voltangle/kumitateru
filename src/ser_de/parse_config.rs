use crate::ser_de::config::app_config::AppConfig;

pub fn parse_config(config: String) -> AppConfig {
    return toml::from_str::<AppConfig>(&*config).unwrap();
}
