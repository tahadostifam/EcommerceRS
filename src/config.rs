use serde::{Deserialize, Serialize};
use std::{env, fs, sync::LazyLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: i32,
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Postgres {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub postgres: Postgres,
    pub redis: Redis,
    pub version: String,
}

static INSTANCE: LazyLock<Config> = LazyLock::new(|| {
    let file_name = if app_env() == "devel" {
        "App.devel.toml"
    } else {
        "App.prod.toml"
    };
    let file_path = format!(
        "{}/config/{}",
        env::current_dir().unwrap().display(),
        file_name
    );
    let data = fs::read_to_string(file_path).unwrap();
    let config: Config = toml::from_str(data.as_str()).unwrap();
    set_database_url(config.postgres.url.clone());
    return config;
});

pub fn read() -> Config {
    INSTANCE.clone()
}

pub fn app_env() -> String {
    let env = env::var("ECOMMERCERS_ENV").unwrap_or_else(|_| "devel".to_string());
    match env.as_str() {
        "devel" | "prod" => env,
        _ => panic!(
            "ECOMMERCERS_ENV mus be one of 'devel', 'prod' but got '{}.",
            env
        ),
    }
}

fn set_database_url(url: String) {
    unsafe { env::set_var("DATABASE_URL", url) };
}
