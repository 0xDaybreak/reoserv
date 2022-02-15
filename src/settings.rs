use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: String,
    pub max_connections: u32,
    pub password_salt: String,
    pub ping_rate: u32,
    pub enforce_sequence: bool,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: String,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct NewCharacter {
    pub spawn_map: u32,
    pub spawn_x: u32,
    pub spawn_y: u32,
    pub spawn_direction: u32,
    pub home: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub new_character: NewCharacter,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("Config.toml"))?;
        s.merge(File::with_name("Config.local.toml").required(false))?;
        s.try_into()
    }
}