use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeSettings {
    pub api_key: String,
    pub api_secret: String,
    pub env: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub stripe: StripeSettings,
}


pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    dotenv::dotenv().ok();

    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    // base config file
    settings.merge(
        config::File::from(configuration_directory.join("base")).required(true)
    )?;

    // env specific config file
    let env: String = std::env::var("ENV").unwrap_or("local".to_string());
    settings.merge(
        config::File::from(configuration_directory.join(env.as_str())).required(true),
    )?;

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    settings.merge(
        config::Environment::with_prefix("app").separator("__")
    )?;

    settings.try_into()
}
