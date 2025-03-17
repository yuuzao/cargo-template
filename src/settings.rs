use config::{Config, File, Environment};
use log::{LevelFilter, info};
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use serde::Deserialize;
use std::io::Write;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub hello: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

// using config to load config.toml to static
pub static CONFIG: Lazy<Settings> = Lazy::new(|| {
    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .add_source(Environment::default().prefix("APP")) // 优先级：env > config.toml
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
