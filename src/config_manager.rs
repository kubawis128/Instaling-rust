#![allow(non_upper_case_globals)]

use configparser::ini::Ini;
use std::fs;
use once_cell::sync::Lazy;

static mut config: Lazy<configparser::ini::Ini> = Lazy::new(||Ini::new());

// Load config to memory
pub fn load_config() {

    unsafe {

        // Read file and parse it
        config.read(fs::read_to_string("./config.conf").unwrap());

    }
    
}

// Get varible from config
pub fn get_from_config(selection: &str, name: &str) -> String {
    
    unsafe{

        config.get(selection, name).unwrap()

    }
}