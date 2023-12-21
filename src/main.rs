mod schema;
mod db;
mod models;

mod ops;

use ops::ff::{
    create_new_key_path,
    get_path_from_key,
    update_key_path,
    delete_key,
    list_keys
};

use clap::{Command, Arg};

fn main() {
    let matches = Command::new("CDF")
        .version("1.0")
        .author("NurramoX")
        .about("Change Directory Fast")
        .subcommand(Command::new("register")
            .about("Registers a key and its corresponding path")
            .arg(Arg::new("key")
                .help("The key to register")
                .required(true))
            .arg(Arg::new("path")
                .help("The corresponding path")
                .required(true)))
        .subcommand(Command::new("list")
            .about("Get the path of the registered key")
            .arg(Arg::new("key")
                .help("Registered key")))
        .subcommand(Command::new("get")
            .about("Get the path of the registered key")
            .arg(Arg::new("key")
                .help("Registered key")
                .required(true)))
        .subcommand(Command::new("delete")
            .about("Delete the path of the registered key")
            .arg(Arg::new("key")
                .help("Registered key")
                .required(true)))
        .subcommand(Command::new("update")
            .about("Updates a key and its corresponding path")
            .arg(Arg::new("key")
                .help("The key to update")
                .required(true))
            .arg(Arg::new("path")
                .help("The corresponding path")
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        Some(("register", sub_m)) => {
            let k = sub_m.get_one::<String>("key").unwrap();
            let p = sub_m.get_one::<String>("path").unwrap();
            create_new_key_path(k, p);
        }
        Some(("get", sub_m)) => {
            let k = sub_m.get_one::<String>("key").unwrap();
            get_path_from_key(k);
        }
        Some(("delete", sub_m)) => {
            let k = sub_m.get_one::<String>("key").unwrap();
            delete_key(k);
        }
        Some(("update", sub_m)) => {
            let k = sub_m.get_one::<String>("key").unwrap();
            let p = sub_m.get_one::<String>("path").unwrap();
            update_key_path(k, p);
        }
        Some(("list", sub_m)) => {
            // Call function to list all keys
            let k = sub_m.get_one::<String>("key").map(|s| s.as_str()).unwrap_or("");
            list_keys(k);
        }
        _ => unreachable!(), // If all subcommands are defined, this shouldn't happen
    }
}
