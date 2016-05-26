extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};

use std::fs::File;
use std::env;
use std::io::prelude::*;

pub fn parse_config() -> Vec<String> {

    // Attempt to find configuration file (~/.terminator.yml).
    let mut path = env::home_dir().unwrap();

    path.push(".terminator.yml");
    let display = path.display();
    if !path.exists() || !path.is_file() {
       panic!("Could not find {}, please check the documentation before using this!", display);
    }

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
