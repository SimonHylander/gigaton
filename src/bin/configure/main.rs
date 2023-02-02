use std::{env::{self, home_dir}, fs::create_dir, path::Path};
fn main() {
    println!("{}", env::consts::OS);


    let h = home_dir();

    let dir = "~/.gigaton";
    let path = Path::new(dir);
    let config_file = ".gigaton/gigaton.yml";

    if !path.is_dir() {
        println!("{} does not exist.", dir);
        create_dir(path).expect("Could not create directory");
    }

    println!("{}", config_file);

    if env::consts::OS == "linux" {}
}
