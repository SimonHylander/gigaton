use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

use config::{load_config, Config};
use ssh2::Session;

use crate::gigaton;

pub fn deploy(src_dir: &String) {
    let config = load_config();

    if config.is_err() {
        println!("Could not load config");
    }

    let c = config.unwrap();

    let gigaton_dir = Path::new(".gigaton");

    if !gigaton_dir.exists() {
        fs::create_dir(gigaton_dir).unwrap();
    }

    let dir_name = src_dir.split("/").last().unwrap();
    let path = format!(".gigaton/{}.zip", dir_name).to_string();
    let dst_file = Path::new(&path);

    gigaton::zip_directory(src_dir, dst_file);

    setup_ssh(&c, dst_file);
}

fn setup_ssh(config: &Config, dst_file: &Path) {
    let tcp = TcpStream::connect(format!("{}:22", config.gigaton.host)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    if config.gigaton.identityfile.is_some() {
        let identity_file = config.gigaton.identityfile.as_ref().unwrap();
        let privatekeypath = std::path::Path::new(identity_file);

        if !privatekeypath.exists() {
            println!("Identity file does not exist");
            return;
        }

        let privatekeydata: &str = &std::fs::read_to_string(privatekeypath).unwrap();

        let passphrase: Option<&str> = match &config.gigaton.passphrase {
            Some(p) => Some(p),
            None => None,
        };

        let id = sess.userauth_pubkey_memory(
            &config.gigaton.username,
            Option::None,
            privatekeydata,
            passphrase,
        );

        if id.is_err() {
            println!("Could not authenticate with identity file");
            return;
        }
    } else {
        // TODO: implement
        // let pw = sess.userauth_keyboard_interactive();
    }

    assert!(sess.authenticated());

    let f = fs::File::open(dst_file);

    if f.is_err() {
        println!("Could not open file");
        return;
    }

    let filename = dst_file.file_name().unwrap().to_str().unwrap();
    let mut buffer = Vec::new();
    let mut file = fs::File::open(dst_file).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    let b: &[u8] = &buffer;

    let remote_path_str = format!("/tmp/gigaton/{}", &filename).to_string();
    let remote_path = Path::new(&remote_path_str);

    println!("Uploading: {}{}", config.gigaton.host, remote_path_str);

    let mut remote_file = sess
        .scp_send(remote_path, 0o644, b.len() as u64, None)
        .unwrap();

    remote_file.write_all(b).unwrap();
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
}
