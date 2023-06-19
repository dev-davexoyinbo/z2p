#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub applicationn_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}