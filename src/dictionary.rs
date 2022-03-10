#![allow(non_upper_case_globals)]

use once_cell::sync::Lazy;
use std::{fs::{self, OpenOptions}};
use crate::config_manager::get_from_config;
use std::io::prelude::*;

static mut dictionary_string: Lazy<String> = Lazy::new(||"".to_string());

// Load file to memory
pub fn load_dict() -> String{

    unsafe{
        
        // Read file to varible as &str
        dictionary_string = Lazy::new(||fs::read_to_string(get_from_config("dictionary","dict_file")).unwrap());
        
        // Return it as String
        dictionary_string.to_string()

    }

}

// Loop over every line in dictionary file and check if varible exists
pub fn read_from_dict(example_usage: String) -> String{

    unsafe{

        // Set temp varible to None if we didn't find anything
        let mut temporary_varible: String = "None".to_string();
        for translate in dictionary_string.split("\n") {

            if translate.contains(example_usage.as_str()) {

                temporary_varible = {
                    let this = translate.split(" $ ").nth(1);
                    match this {
                        Some(val) => val,
                        None => "None",
                    }
                }.to_string();
                
            }

        }

        // Return String varible
        temporary_varible

    }
    
}

// Append String to text file
pub fn write_to_dict(word_to_append: String){

    // Open file for write
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(get_from_config("dictionary","dict_file"))
    .unwrap();

    // Try to write to it
    if let Err(e) = writeln!(file, "{}", format!("{}", word_to_append)) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // Refresh file in memory
    load_dict();
}