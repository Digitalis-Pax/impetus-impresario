use std::{env, fs, path::PathBuf};

use config::{Config, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::logging::LoggingConfiguration;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Configuration {
    pub oauth_credential: Option<OAuthCredential>,
    pub datastore: DataStoreConfiguration,
    pub logging: LoggingConfiguration,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OAuth {
    pub auth_code_pkce: OAuthCredential,
    pub client_credential: OAuthCredential,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct OAuthCredential {
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DataStoreConfiguration {
    pub store_type: DataStoreType,
    pub host: String,
    pub port: u64,
    pub schema: String,
    pub user: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum DataStoreType {
    MariaDB,
}

impl Configuration {
    pub fn load() -> Self {
        const CONFIGURATION_FILE_NAME: &str = "config.json";

        let project_dirs =
            ProjectDirs::from("org", "digitalis-pax", env!("CARGO_PKG_NAME")).unwrap();

        let direct_configuration_path: PathBuf = env::current_dir()
            .unwrap()
            .join("config")
            .join(CONFIGURATION_FILE_NAME);

        let system_configuration_path: PathBuf =
            ["/etc", env!("CARGO_PKG_NAME"), CONFIGURATION_FILE_NAME]
                .iter()
                .collect();

        let local_configuration_path: PathBuf = project_dirs
            .config_dir()
            .join(CONFIGURATION_FILE_NAME)
            .to_path_buf();

        if !fs::exists(&direct_configuration_path).unwrap()
            && !fs::exists(&system_configuration_path).unwrap()
            && !fs::exists(&local_configuration_path).unwrap()
        {
            panic!(
                "Unable to find a configuration at:\n{}\n{}\n{}",
                direct_configuration_path.to_str().unwrap(),
                system_configuration_path.to_str().unwrap(),
                local_configuration_path.to_str().unwrap(),
            );
        }

        let config_builder = Config::builder()
            .set_default("logging.enable_console", true)
            .unwrap()
            .set_default("logging.root_level", log::Level::Info.as_str())
            .unwrap()
            .add_source(
                File::with_name(direct_configuration_path.to_str().unwrap()).required(false),
            )
            .add_source(
                File::with_name(system_configuration_path.to_str().unwrap()).required(false),
            )
            .add_source(File::with_name(local_configuration_path.to_str().unwrap()).required(false))
            .build()
            .unwrap();

        config_builder.try_deserialize().unwrap()
    }
}
