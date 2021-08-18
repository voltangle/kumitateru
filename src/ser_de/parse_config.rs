use crate::ser_de::config::app_config::AppConfig;
use anyhow::Context;

pub fn parse_config(config: &str) -> AppConfig {
    return toml::from_str::<AppConfig>(config).with_context(|| "Unable to parse package.toml").unwrap();
}
