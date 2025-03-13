use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
