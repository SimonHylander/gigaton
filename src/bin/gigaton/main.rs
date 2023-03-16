use std::env;

mod deploy;
mod gigaton;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No command specified");
        return;
    }

    let query = &args[1];

    if query != "deploy" {
        println!("Unknown command");
        return;
    }

    if query == "deploy" {
        if args.len() < 3 {
            println!("No target directory specified");
            return;
        }

        let dir = &args[2];
        let cwd = &env::current_dir().unwrap().to_str().unwrap().to_string();

        if dir == "." {
            deploy::deploy(cwd);
        } else {
            let mut src_dir = cwd.to_string();
            src_dir.push_str("/");
            src_dir.push_str(dir);
            deploy::deploy(&src_dir);
        }
    }
}
