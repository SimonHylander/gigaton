mod deploy;
mod gigaton;

fn main() {
    /* let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        println!(
            "Usage: {} <source_directory> <destination_zipfile>",
            args[0]
        );
        return 1;
    } */

    /* let src_dir = &*args[1];
    let dst_file = &*args[2]; */

    let src_dir = "/home/simonhylander/development/web/gigaton/app";
    let dst_file = "/home/simonhylander/development/web/gigaton/zip/app.zip";

    gigaton::zip_directory(src_dir, dst_file);

    let user = "";
    let server = "";
    let remote_path = "";
    let local_path = "";

    deploy::setup_ssh(user, server, remote_path, local_path);
}
