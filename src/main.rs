use std::env;

mod deploy;
mod gigaton;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    if query == "deploy" {
        if (args.len() < 3) {
            println!("Please provide a target for deployment.");
            return;
        }

        deploy::deploy(&args[2]);
    }

    /* let src_dir = "/home/simonhylander/development/web/gigaton/app";
    let dst_file = "/home/simonhylander/development/web/gigaton/zip/app.zip";

    gigaton::zip_directory(src_dir, dst_file);

    let user = "";
    let server = "";
    let remote_path = "";
    let local_path = "";

    deploy::setup_ssh(user, server, remote_path, local_path); */
}
