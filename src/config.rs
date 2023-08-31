use config::{builder::DefaultState, Config, ConfigBuilder, Environment};
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::{cli::Cli, error::AppError};

const ENV_PREFIX: &str = "app";

#[derive(Debug, Serialize, Deserialize)]
/// All applications related configuration should go here
pub struct AppConfig {
    /// Determines output logging level
    pub log_level: usize,
    /// If true, do not do any actual work
    pub dry_run: bool,
    pub server_port: String,
    pub server_host: String,
}

impl AppConfig {
    pub fn new(cli_args: &Cli) -> Result<Self> {
        let log_level = match cli_args.debug {
            0 => 1,
            0..=3 => cli_args.debug + 2,
            level if level > 3 => 5,
            _ => unreachable!(),
        };
        // Override environment variables as required

        // Get the default config path
        let s: ConfigBuilder<DefaultState>;
        s = Config::builder()
            // e.g. `APP_USER=alice ./target/app` would set the 'user' key
            .add_source(Environment::with_prefix(ENV_PREFIX))
            // NOTE: Define defaults here
            .set_default("log_level", log_level)?
            .set_default("dry_run", false)?
            .set_default("server_port", "8080")?
            .set_default("server_host", "0.0.0.0")?;

        // Build the config
        match s.build()?.try_deserialize::<AppConfig>() {
            Ok(v) => Ok(v),
            Err(e) => return Err(AppError::Config(e)),
        }
    }
}
