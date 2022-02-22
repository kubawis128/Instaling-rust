// Import my modules
mod config_manager;
mod dictionary;

// Import external modules
extern crate reqwest;
extern crate json;
extern crate colored;

// Choose what functions would we use
use rustlate::{self, Translator};
use colored::Colorize;
use std::{collections::HashMap, time::SystemTime};
use std::{thread, time};
use crate::config_manager::get_from_config;

// Main function
fn main() {

    println!("{}","Starting Instaling rust bot".blue().bold());

    // Read config file
    config_manager::load_config();

    // Init translator
    let translator_struct = Translator{to: "de",from: "pl"}; // TODO: implement reading from config (I'm too stupid for this) smth like: get_from_config("translator","from") and get_from_config("translator","to")

    // Create client
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            attempt.stop()
        }))
        .build()
        .unwrap();

    // Get basic cookies
    client.get("https://instaling.pl/teacher.php?page=login").send().unwrap();

    // Create Map for form request
    let mut map = HashMap::new();
    let login = &get_from_config("account","login");
    let password = &get_from_config("account","passwd");

    map.insert("from", "");
    map.insert("action", "login");
    map.insert("log_password", &password);
    map.insert("log_email", &login);
    
    // Login
    client.post("https://instaling.pl:443/teacher.php?page=teacherActions")
        .form(&map)
        .send()
        .unwrap();

    println!("{}","Succesfully logged in".green().bold());
    
    // Get child_id
    let res = client.get("https://instaling.pl:443/learning/dispatcher.php?from=")
        .send()
        .unwrap();

    let student_id = res.headers().values().nth(9).unwrap().to_str().unwrap().split("=").nth(1).unwrap();

    // Idk if it is needed
    client.get("https://instaling.pl:443/student/pages/mainPage.php?student_id=".to_string() + student_id)
        .send()
        .unwrap();

    // Clear map
    map.clear();

    // And set new values
    map.insert("child_id", student_id);
    map.insert("repeat", "");
    map.insert("start", "");
    map.insert("end", "");
    
    // Init Learning session
    client.post("https://instaling.pl:443/ling2/server/actions/init_session.php")
        .form(&map)
        .send()
        .unwrap();
    
    // Main loop of program
    loop {

        // Create second map for this loop
        let mut map1 = HashMap::new();

        // Get unix timestamp
        let timestamp = &SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string();

        map1.insert("child_id", student_id);
        map1.insert("date",timestamp);

        // Generate next word
        let res = client.post("https://instaling.pl:443/ling2/server/actions/generate_next_word.php")
            .form(&map1)
            .send()
            .unwrap();

        // Get response from instaling server and parse it so we can use parsed["example"] intead of manually parsing json
        let parsed = json::parse(res.text().unwrap().as_str()).unwrap();
        
        // If summary isn't null then we didin't finish session yet
        if !parsed["summary"].is_null() {

            println!("{}","I'm done".purple());
            break;

        } else {

            println!("{}","Doing next excercise".blue());

        }

        // Get usage example and try to find it in dictionary
        let example_use =  parsed["usage_example"].to_string();
        let read_from_dictionary = dictionary::read_from_dict(example_use.clone());
        
        // Get word_id
        let word_id: &str = &parsed["id"].to_string();

        // Clear map and set form varibles
        map1.clear();
        map1.insert("child_id", student_id);
        map1.insert("word_id", word_id);
        map1.insert("version", "C65E24B29F60B1221EC23D979C9707DE");

        // read_from_dictionary will return None as String if the example usage isn't found else it retuen answer
        let mut answear = read_from_dictionary.clone();
        let mut bylo_tlumaczone: bool = false;

        // Check for None
        if read_from_dictionary != "None".to_string() {

            // Use answer from dictionary 
            let str3: &str = &answear;
            answear = str3.to_string();

        } else {

            bylo_tlumaczone = true;
            let polnish = parsed["translations"].to_string();
            answear = translator_struct.translate(&polnish).unwrap();

            // if answer contains , than pick first word
            if answear.contains(",") {
                answear = answear.split(",").nth(0).unwrap().to_string();
            }

        }

        // Add answer to form request
        map1.insert("answer", &answear);

        // Get from config how long do we need to sleep
        let sleep = get_from_config("timing","sleep_per_letter").parse::<u64>().unwrap() * answear.len() as u64;
        let sleep = sleep + get_from_config("timing","sleep_before_sending").parse::<u64>().unwrap();

        // Than pause the thread for that amount of time
        thread::sleep(time::Duration::from_millis(sleep));

        // Finally send anwears to instaling
        let res = client.post("https://instaling.pl:443/ling2/server/actions/save_answer.php")
                .form(&map1)
                .send()
                .unwrap();

        // Parse response json
        let parsed = json::parse(res.text().unwrap().as_str()).unwrap();
        let grade = parsed["grade"].to_string();

        // And check if we did it corectly
        if grade != "3" && grade != "0" {

            if bylo_tlumaczone{

                println!("{} {}","Was translation successfull?:".yellow(),"Yes".green().bold());
                
                // If translation was succesfull then write it to dictionary for later use
                dictionary::write_to_dict(format!("{} $ {}",example_use,parsed["answershow"].to_string()));  

            } else{

                println!("{}","Found it in cache".green());  

            }
            
        } else{

            println!("{} {}","Was translation successfull?:".yellow(),"No".red().bold());
            // If translation was unsuccesfull then write answer that was sent in response to dictionary for later use
            dictionary::write_to_dict(format!("{} $ {}",example_use,parsed["answershow"].to_string()));

        }
    }
}