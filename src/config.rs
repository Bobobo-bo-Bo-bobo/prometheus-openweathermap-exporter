use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub units: Option<String>,
    pub api_key: String,
    pub locations: Vec<String>,
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let unparsed = fs::read_to_string(f)?;
    let mut config: Configuration = serde_yaml::from_str(unparsed.as_str())?;

    validate_configuration(&config)?;

    Ok(config)
}

fn validate_configuration(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    if cfg.api_key.is_empty() {
        bail!("Missing API key");
    }
    
    if cfg.locations.is_empty() {
        bail!("No locations to query");
    }

    Ok(())
}
