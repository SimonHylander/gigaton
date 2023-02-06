use std::{
    env,
    fs::{create_dir, File},
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

fn main() {
    println!("{}", env::consts::OS);

    let config = load_config();

    if config.is_err() {
        println!("Could not load config");
    }

    println!("{:?}", config.unwrap());

    // TODO: cli prompt to add new targets
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub targets: Mapping,
}

pub fn load_config() -> std::io::Result<Config> {
    let config = dirs::config_dir().unwrap();
    let path = &Path::new(config.as_path()).join("gigaton");

    if !path.is_dir() {
        create_dir(path).expect("Could not create directory");
    }

    let config_file = &Path::new(path).join("gigaton.yaml");
    println!("{:?}", config_file.exists());

    if !config_file.exists() {
        println!("Creating config file:");
        File::create(config_file)?;

        let c = Config {
            targets: Mapping::new(),
        };

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_file)
            .expect("Couldn't open file");

        serde_yaml::to_writer(f, &c).unwrap();

        return Ok(c);
    }

    let f = std::fs::File::open(config_file).unwrap();
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read config");

    return Ok(scrape_config);
}
