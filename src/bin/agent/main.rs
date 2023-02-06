use config::{load_config, Config};
use notify::{
    event, Config as NotifyConfig, EventKind, PollWatcher, RecommendedWatcher, RecursiveMode,
    Watcher, WatcherKind,
};
use std::{collections::HashMap, io::Result, path::Path, time::Duration};

fn main() {
    let cfg: Result<Config> = load_config();

    if cfg.is_err() {
        println!("Could not load config");
    }

    let config: Config = cfg.unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    // This example is a little bit misleading as you can just create one Config and use it for all watchers.
    // That way the pollwatcher specific stuff is still configured, if it should be used.
    let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        // custom config for PollWatcher kind
        // you
        let ncfg = NotifyConfig::default().with_poll_interval(Duration::from_secs(1));
        Box::new(PollWatcher::new(tx, ncfg).unwrap())
    } else {
        // use default config for everything else
        Box::new(RecommendedWatcher::new(tx, NotifyConfig::default()).unwrap())
    };

    let target_map: HashMap<String, String> = HashMap::new();

    config.targets.iter().for_each(|target| {
        let key = target.0.as_str().unwrap();
        let target_path = Path::new(key);

        println!("{}", target_path.display().to_string());

        if !target_path.is_dir() {
            println!("Target path does not exist");
            std::process::exit(1);
        }

        // watch some stuff
        watcher
            .watch(Path::new(target_path), RecursiveMode::Recursive)
            .unwrap();
    });

    // just print all events, this blocks forever
    for e in rx {
        if e.is_ok() {
            let event = e.unwrap();
            // println!("{:?}", event.kind);

            let is_created = event.kind == EventKind::Create(event::CreateKind::File);
            let is_modified =
                event.kind == EventKind::Modify(event::ModifyKind::Name(event::RenameMode::Both));

            if is_created {
                if event.paths.len() > 0 {
                    let path = event.paths[0].to_str().unwrap();
                    println!("File created: {}", path);
                    // Only handle directory, specific files will be too complex

                    let target_path: String = config
                        .targets
                        .iter()
                        .map(|target| target.0.as_str().unwrap())
                        .collect();

                    println!("Target path: {}", target_path);

                    if path.eq(target_path.as_str()) {
                        println!("File is a target, deploying...");
                    }
                }
            }

            if is_modified {
                println!("File modified");
            }
        }
    }
}

// fn authentication() {}

//  * This function is called before the deployment of the application.
// fn pre_deploy() {}

//  * This function is called after the deployment of the application.
// fn post_deploy() {}
