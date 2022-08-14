#![allow(non_upper_case_globals)]

use configparser::ini::Ini;
use std::{fs};
use once_cell::sync::Lazy;

static mut config: Lazy<configparser::ini::Ini> = Lazy::new(||Ini::new());

// Load config to memory
pub fn load_config() {

    unsafe {

        // Read file and parse it
        let file = fs::read_to_string("./config.conf");
        let file_string = match file {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        match config.read(file_string) {
            Ok(file) => file,
            Err(error) => panic!("Problem reading the config file: {:?}", error),
        };

    }
    
}

// Get varible from config
pub fn get_from_config(selection: &str, name: &str) -> String {
    
    unsafe{

        config.get(selection, name).unwrap()

    }
}

// Set varible to config
pub fn set_to_config(selection: &str, name: &str, value: Option<&str>) {
    
    unsafe{
        config.setstr(selection, name, value);
        config.write("./config.conf").unwrap();
        load_config();
    }
}

pub fn get_from_config_static(selection: &str, name: &str) -> &'static str {

    unsafe{
        string_to_static_str(config.get(selection, name).unwrap())
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}