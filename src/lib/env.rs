use clap::{Parser, ValueEnum};
use std::fmt::Display;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum AppEnv {
    #[default]
    Development,
    Production,
}

impl From<String> for AppEnv {
    fn from(value: String) -> Self {
        match value.as_str() {
            "development" => AppEnv::Development,
            "production" => AppEnv::Production,
            _ => AppEnv::Development,
        }
    }
}

impl Display for AppEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppEnv::Development => write!(f, "development"),
            AppEnv::Production => write!(f, "production"),
        }
    }
}

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub port: String,

    #[clap(env, default_value = "development", value_enum)]
    pub env: AppEnv,
}
