use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use bencher_json::{
    config::{JsonDatabase, JsonLogging, JsonServer, LogLevel, ServerLog},
    JsonConfig,
};
use tracing::{error, info};
use url::Url;

use crate::ApiError;

pub mod config_tx;

pub const API_NAME: &str = "Bencher API";

pub const BENCHER_CONFIG: &str = "BENCHER_CONFIG";
pub const BENCHER_CONFIG_PATH: &str = "BENCHER_CONFIG_PATH";

const DEFAULT_CONFIG_PATH: &str = "bencher.json";
#[cfg(debug_assertions)]
const DEFAULT_ENDPOINT_STR: &str = "http://localhost:3000";
#[cfg(not(debug_assertions))]
const DEFAULT_ENDPOINT_STR: &str = "https://bencher.dev";
// Dynamic and/or Private Ports (49152-65535)
// https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml?search=61016
const DEFAULT_PORT: u16 = 61016;
// 1 megabyte or 1_048_576 bytes
const DEFAULT_MAX_BODY_SIZE: usize = 1 << 20;
const DEFAULT_DB_PATH: &str = "data/bencher.db";

lazy_static::lazy_static! {
    static ref DEFAULT_ENDPOINT: Url = DEFAULT_ENDPOINT_STR.parse().expect(&format!("Failed to parse default endpoint: {DEFAULT_ENDPOINT_STR}"));
    static ref DEFAULT_BIND_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), DEFAULT_PORT);
}

#[cfg(debug_assertions)]
lazy_static::lazy_static! {
    static ref DEFAULT_SECRET_KEY: String = "DO_NOT_USE_THIS_IN_PRODUCTION".into();
}
#[cfg(not(debug_assertions))]
lazy_static::lazy_static! {
    static ref DEFAULT_SECRET_KEY: String = uuid::Uuid::new_v4().to_string();
}

#[derive(Debug, Clone)]
pub struct Config(pub JsonConfig);

impl Config {
    pub fn path() -> PathBuf {
        std::env::var(BENCHER_CONFIG_PATH)
            .unwrap_or_else(|e| {
                info!("Failed to find \"{BENCHER_CONFIG_PATH}\": {e}");
                info!("Defaulting \"{BENCHER_CONFIG_PATH}\" to: {DEFAULT_CONFIG_PATH}");
                DEFAULT_CONFIG_PATH.into()
            })
            .into()
    }

    pub async fn load_file() -> Result<Self, ApiError> {
        let path = Self::path();

        let config_file = tokio::fs::read(&path).await.map_err(|e| {
            info!("Failed to open config file at {}: {e}", path.display());
            ApiError::OpenConfigFile(path.clone())
        })?;

        let json_config = serde_json::from_slice(&config_file).map_err(|e| {
            info!("Failed to parse config file at {}: {e}", path.display());
            ApiError::ParseConfigFile(path.clone())
        })?;
        info!(
            "Loaded config from {}: {}",
            path.display(),
            serde_json::json!(json_config)
        );

        Ok(Self(json_config))
    }

    pub async fn load_env() -> Result<Self, ApiError> {
        let config_str = std::env::var(BENCHER_CONFIG).map_err(|e| {
            info!("Failed to find \"{BENCHER_CONFIG}\": {e}");
            ApiError::MissingEnvVar(BENCHER_CONFIG.into())
        })?;

        let json_config = serde_json::from_str(&config_str).map_err(|e| {
            info!("Failed to parse config string from \"{BENCHER_CONFIG}\": {e}");
            ApiError::ParseConfigString(config_str.clone())
        })?;
        info!(
            "Loaded config from \"{BENCHER_CONFIG}\": {}",
            serde_json::json!(json_config)
        );

        Self::write(config_str.as_bytes()).await?;

        return Ok(Self(json_config));
    }

    pub async fn load_or_default() -> Self {
        if let Ok(config) = Self::load_file().await {
            return config;
        }

        if let Ok(config) = Self::load_env().await {
            return config;
        }

        let json_config = Self::default();
        info!("Using default config: {}", serde_json::json!(json_config.0));
        json_config
    }

    pub async fn write(config: impl AsRef<[u8]>) -> Result<(), ApiError> {
        let path = Self::path();

        tokio::fs::write(&path, config).await.map_err(|e| {
            error!("Failed to write config file at {}: {e}", path.display());
            ApiError::WriteConfigFile(path)
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self(JsonConfig {
            endpoint: DEFAULT_ENDPOINT.clone(),
            secret_key: Some(DEFAULT_SECRET_KEY.clone()),
            server: JsonServer {
                bind_address: *DEFAULT_BIND_ADDRESS,
                request_body_max_bytes: DEFAULT_MAX_BODY_SIZE,
                tls: None,
            },
            database: JsonDatabase {
                file: DEFAULT_DB_PATH.into(),
            },
            smtp: None,
            logging: JsonLogging {
                name: API_NAME.into(),
                log: ServerLog::StderrTerminal {
                    level: LogLevel::Info,
                },
            },
        })
    }
}