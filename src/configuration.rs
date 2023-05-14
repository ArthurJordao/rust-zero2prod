use config::{Config, ConfigError, File};
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Option<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?;
    settings.try_deserialize()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let password_string = self
            .password
            .as_ref()
            .map(|password| format!(":{}", password))
            .unwrap_or(String::from(""));
        format!(
            "postgres://{}{}@{}:{}/{}",
            self.username, password_string, self.host, self.port, self.database_name
        )
    }
}
