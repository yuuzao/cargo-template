use config::{Config, File, Environment};
use log::{LevelFilter, info};
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use serde::Deserialize;
use std::io::Write;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub hello: Hello,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Hello {
    pub value: String,
}

// using config to load config.toml to static
pub static CONFIG: Lazy<Settings> = Lazy::new(|| {
    let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "dev".to_string());
    if run_mode == "dev" {
        dotenv::from_filename(".env.dev").ok();
    } else if run_mode == "prod" {
        dotenv::from_filename(".env").ok();
    }
    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .add_source(Environment::default().separator("__")) // 优先级：env > config.toml, 嵌套结构体中环境变量：APP_SERVER_PORT
        .build()
        .unwrap();
    config.try_deserialize::<Settings>().unwrap()
});

pub struct Initializer;

impl Initializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(self) {
        self.init_logger();
        let config = CONFIG.clone();
        info!("settings initialized: {:?}", config);
    }

    pub fn init_logger(&self) -> &Self {
        let mut builder = env_logger::builder();
        builder
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} - {} {} -- {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    match record.level() {
                        log::Level::Error => "ERROR".red().to_string(),
                        log::Level::Info => "INFO".green().to_string(),
                        _ => "DEBUG".blue().to_string(),
                    },
                    record.target().yellow(),
                    record.args()
                )
            })
            .filter(None, LevelFilter::Info)
            .init();
        self
    }
}
