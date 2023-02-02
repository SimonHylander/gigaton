use std::{
    env,
    fs::{create_dir, File},
    path::Path,
};
fn main() {
    println!("{}", env::consts::OS);

    let config = dirs::config_dir().unwrap();
    let path = Path::new(config.as_path());

    if !path.is_dir() {
        create_dir(path).expect(
            "Could not c
        reate directory",
        );
    }

    let config_file = Path::new(path).join("gigatom.yaml");

    if !config_file.exists() {
        let mut file = File::open(config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        assert_eq!(contents, "Hello, world!");
    }

    if env::consts::OS == "linux" {}
}
