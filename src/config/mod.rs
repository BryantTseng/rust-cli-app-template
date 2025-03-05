use serde::Deserialize;
use std::fs;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct CliConfig {
    pub global: Global,
}

#[derive(Debug, Deserialize)]
pub struct Global {
    pub debug: bool,
}

impl CliConfig {
    pub fn init(path: &str) -> Self {
        info!("Reading config from {}", path);

        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        let cli_config: CliConfig = toml::from_str(&contents).unwrap();

        cli_config
    }
}
