use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct AppConfig {
    location: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Database {
    url: Option<String>, // FIXME: Remove option when database is added
}

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    config: AppConfig,
    database: Database,
}

#[derive(Debug, Error)]
pub enum SettingsCreateError {
    #[error("Error while parsing config")]
    ConfigError(#[from] ConfigError),
}

impl Settings {
    pub fn init(location: &str) -> Result<Self, SettingsCreateError> {
        let config = Config::builder()
            .add_source(File::with_name(location))
            .add_source(Environment::default())
            .set_override("config.location", location)?
            .build()?;

        let settings = config.try_deserialize()?;

        Ok(settings)
    }
}
