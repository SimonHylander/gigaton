use std::{env, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub gigaton: ConfigBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBase {
    pub host: String,
    pub username: String,
    pub passphrase: Option<String>,
    pub identityfile: Option<String>,
}

pub fn load_config() -> std::io::Result<Config> {
    let cwd = env::current_dir();

    if cwd.is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not determine working directory",
        ));
    }

    let config_file = &Path::new(cwd.unwrap().as_path()).join("gigaton.yaml");
    let file_res = std::fs::File::open(config_file);

    if file_res.is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not open gigaton.yaml",
        ));
    }

    let file = file_res.unwrap();

    let scrape_config: Config = serde_yaml::from_reader(file).expect("Could not read config");

    return Ok(scrape_config);
}
