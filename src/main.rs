extern crate uu_shred;
extern crate walkdir;
extern crate yaml_rust;

use walkdir::{WalkDir};
use yaml_rust::{YamlLoader, Yaml};
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

fn parse_config(path: PathBuf) -> Vec<String> {

    let display = path.display();
    let mut file = match File::open(&path) {
        Err(_) => panic!("Couldn't open {}", display),
        Ok(file) => file,
    };

    let mut tmp = String::new();
    let configuration: Vec<Yaml> = match file.read_to_string(&mut tmp) {
        Err(_) => panic!("Couldn't read {}", display),
        Ok(_) => YamlLoader::load_from_str(&tmp).unwrap(),
    };

    let main_config = &configuration[0];
    if main_config["files"].is_badvalue() || main_config["files"][0].is_badvalue() {
        panic!("Cannot parse {}", display)
    };

    main_config["files"]
        .as_vec()
        .unwrap()
        .iter()
        .filter_map(|e| e.as_str())
        .map(|e| String::from(e))
        .collect::<Vec<String>>()
}

fn traverse_directory(path: &Path) -> Vec<String> {

    // Traverse given path with WalkDir.
    WalkDir::new(path.to_str().unwrap())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| String::from(e.path().to_str().unwrap()))
        .collect::<Vec<String>>()
}

fn main() {

    // Begin building console arguments.
    let mut args = Vec::<String>::new();

    // Since we're pretending to be a console, the first argument is a string
    //   containing the entire command.  Shred ignores it, so this is blank.
    args.push(String::new());

    // To decrease recoverability, we want to zero the file.
    args.push(String::from("--zero"));

    // Ultimately, we want to delete the file.
    args.push(String::from("--remove"));

    // Attempt to find home directory.
    let home = env::home_dir().unwrap();

    // Attempt to find terminator configuration file.
    let mut path = home;
    path.push(".terminator.yml");
    if !path.exists() || !path.is_file() {
       panic!("Could not find {}, please check the documentation before using this!", path.display());
    }

    // Load and parse configuration file, load up valid paths.
    let main_config = parse_config(path);
    let paths = main_config
        .iter()
        .map(|e| Path::new(e.as_str()))
        .filter(|e| e.exists())
        .collect::<Vec<_>>();

    // Iterate over paths, and recurse for directories.
    for current_path in paths {
        if current_path.is_file() { 
            let file = String::from(current_path.to_str().unwrap());
            args.push(file);
        } else if current_path.is_dir() {
            let mut files: Vec<String> = traverse_directory(current_path);
            args.append(&mut files);
        }
    }

    // Finally, defer to shred.
    std::process::exit(uu_shred::uumain(args));
}
