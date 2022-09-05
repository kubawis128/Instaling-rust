// Import my modules
use crate::config_manager;
use crate::dictionary;
use crate::translator_patched::Translator;

// Import external modules
extern crate reqwest;
extern crate json;
extern crate colored;

// Choose what functions would we use
use colored::Colorize;
use gtk::glib::timeout_future_seconds;
use std::{collections::HashMap, time::SystemTime};
use crate::config_manager::get_from_config;
use rand::Rng;

#[derive(Clone)]
pub struct HandlerStruct {
    translator_struct: Translator,
    client: reqwest::blocking::Client,
    map: HashMap<String, String>,
    pub student_id: String,
    pub dialog_show: bool,
    pub dialog_title: String,
    pub dialog_message: String
}
pub struct Response {
    pub succ: bool,
    pub quesion: String,
    pub approx: String,
    pub answear: String,
    pub pol_answer: String,
    pub dialog_show: bool,
    pub dialog_title: String,
    pub dialog_message: String,
    pub ignore: bool
}

impl Response {
    fn new() -> Response {
        Response {
            succ: false,
            quesion: "".to_string(),
            approx: "".to_string(),
            answear: "".to_string(),
            pol_answer: "".to_string(),
            dialog_show: false,
            dialog_title: "".to_string(),
            dialog_message: "".to_string(),
            ignore: false
        }
    }
}

pub fn handler_init() -> HandlerStruct{

    config_manager::load_config();

    // Init translator
    let translator_struct = Translator{to: config_manager::get_from_config_static("translator","to"),from: config_manager::get_from_config_static("translator","from")}; // TODO: implement reading from config (I'm too stupid for this) smth like: get_from_config("translator","from") and get_from_config("translator","to")
    
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            attempt.stop()
        }))
        .build()
        .unwrap();

    let map = HashMap::new();

    let login = get_from_config("account","login").clone();
    let password = get_from_config("account","passwd");

    let mut hs = HandlerStruct{
            translator_struct: translator_struct,
            client: client,
            map: map,
            student_id: "to be filled".to_string(),
            dialog_show: false,
            dialog_title: "".to_string(),
            dialog_message: "".to_string()
    };

    hs.map.insert("from".to_string(), "".to_string());
    hs.map.insert("action".to_string(), "login".to_string());
    hs.map.insert("log_password".to_string(), password);
    hs.map.insert("log_email".to_string(), login);

    

    // Login
    // To jest dramat
    // Brzydkie w chuj i kopiuj-wklej
    // ale działa więc ¯\_(ツ)_/¯
    let post = || -> Result<reqwest::blocking::Response, reqwest::Error> {
        Ok(hs.client.post("https://instaling.pl:443/teacher.php?page=teacherActions")
        .form(&hs.map)
        .send()?)
    };

    match post() {
        Ok(ok) => ok,
        Err(e) => {return send_dialog_handler_init(hs,e)}
    };

        
    let post = || -> Result<reqwest::blocking::Response, reqwest::Error> {
        Ok(hs.client.post("https://instaling.pl:443/teacher.php?page=teacherActions")
        .form(&hs.map)
        .send()?)
    };
    match post() {
        Ok(ok) => ok,
        Err(e) => {return send_dialog_handler_init(hs,e)}
    };


    let get = || -> Result<reqwest::blocking::Response, reqwest::Error> {
        Ok(hs.client.get("https://instaling.pl:443/learning/dispatcher.php?from=")
        .send()?)
    };

    let res = match get() {
        Ok(ok) => ok,
        Err(e) => {return send_dialog_handler_init(hs,e)}
    };

    let student_id_tmp = res
        .headers()
        .values()
        .find(|&x| x.to_str().unwrap().contains("student_id"));


    if !student_id_tmp.is_none(){

        hs.student_id = student_id_tmp
        .unwrap()
        .to_str()
        .unwrap_or("0=0")
        .split("=")
        .nth(1)
        .unwrap()
        .to_string();

    }else {

        hs.student_id = "bruh".to_string();

    }

    // Idk if it is needed
    let get = || -> Result<reqwest::blocking::Response, reqwest::Error> {
        Ok(hs.client.get("https://instaling.pl:443/student/pages/mainPage.php?student_id=".to_string() + &hs.student_id)
        .send()?)
    };

    match get() {
        Ok(ok) => ok,
        Err(e) => {return send_dialog_handler_init(hs,e)}
    };

    // Clear map
    hs.map.clear();

    // And set new values
    hs.map.insert("child_id".to_string(), hs.student_id.clone());
    hs.map.insert("repeat".to_string(), "".to_string());
    hs.map.insert("start".to_string(), "".to_string());
    hs.map.insert("end".to_string(), "".to_string());

    // Init Learning session
    let post = || -> Result<reqwest::blocking::Response, reqwest::Error> {
        Ok(hs.client.post("https://instaling.pl:443/ling2/server/actions/init_session.php")
        .form(&hs.map)
        .send()?)
    };
    match post() {
        Ok(ok) => ok,
        Err(e) => {return send_dialog_handler_init(hs,e)}
    };

    hs
}


pub async fn loop_de_loop(hr: HandlerStruct) -> Response{

    // Get unix timestamp
    let timestamp = &SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_millis();

    let mut map1 = HashMap::new();

    map1.insert("child_id", hr.student_id.clone());
    map1.insert("date",timestamp.to_string());
    
    let post = || -> Result<reqwest::blocking::Response, reqwest::Error> {
        Ok(hr.client.post("https://instaling.pl:443/ling2/server/actions/generate_next_word.php")
        .form(&map1)
        .send()?)
    };
    
    let res = match post() {
        Ok(ok) => ok,
        Err(e) => {return send_dialog_response_handler(e)}
    };
    
    // Get response from instaling server and parse it so we can use parsed["example"] intead of manually parsing json
    let parsed_check = json::parse(res.text()
        .unwrap()
        .as_str());

    let parsed;

    match parsed_check {
        Ok(t) => parsed = t, // Everything is ok
        Err(_e) => panic!("{}","You might have been banned F".red().bold()) // Something went wrong might indicate ban: thanks Nicolass1000 for your sacriface // TODO: signal message might be banned
    }

    // If summary isn't null then we didin't finish session yet
    if !parsed["summary"].is_null() {

        // Get unix timestamp
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string();
        map1.insert("child_id", hr.student_id.clone());
        map1.insert("date",timestamp);

        // Get grades
        let post = || -> Result<reqwest::blocking::Response, reqwest::Error> {
            Ok(hr.client.post("https://instaling.pl/ling2/server/actions/grade_report.php")
            .form(&map1)
            .send()?)
        };
        
        let res = match post() {
            Ok(ok) => ok,
            Err(e) => {return send_dialog_response_handler(e)}
        };

        let status;
        let parsed = json::parse({
            status = res.text_with_charset("utf-8");
            match status {
                Ok(t) => t,
                Err(e) => format!("BRUH: {:?}", e),
            }
        }.as_str()).unwrap();

        let mut response_handler: Response = Response::new();
        response_handler.dialog_show = true;
        response_handler.dialog_title = "Done!".to_string();
        response_handler.dialog_message = format!("Work days done: {0}\nPrevious mark: {1}\nCurrent mark: {2}",parsed["work_week_days"],parsed["prev_mark"],parsed["current_mark"]);
        response_handler

    } else {

        // Get usage example and try to find it in dictionary
        let example_use =  parsed["usage_example"].to_string();
        let read_from_dictionary = dictionary::read_from_dict(example_use.clone());

        if example_use.clone() == "null"{
            let mut response_handler: Response = Response::new();
            response_handler.ignore = true;
            return response_handler;
            //continue;
        }

        // Get word_id
        let word_id: &str = &parsed["id"].to_string();

        // Clear map and set form varibles
        map1.clear();
        map1.insert("child_id", hr.student_id);
        map1.insert("word_id", word_id.to_string());
        map1.insert("version", "C65E24B29F60B1221EC23D979C9707DE".to_string());

        // read_from_dictionary will return None as String if the example usage isn't found else it retuen answer
        let mut answear = read_from_dictionary.clone();
        let polnish = parsed["translations"].to_string();
        let mut bylo_tlumaczone: bool = false;

        // Check for None
        if read_from_dictionary != "None".to_string() {

            // Use answer from dictionary 
            let str3: &str = &answear;
            answear = str3.to_string();
        } else {
            bylo_tlumaczone = true;
            let polnish = parsed["translations"].to_string();
            answear = hr.translator_struct.translate(&polnish).unwrap();

            // if answer contains , than pick first word
            if answear.contains(",") {
                answear = answear.split(",").nth(0).unwrap().to_string();
            }
        }

        // Add answer to form request
        map1.insert("answer", answear.clone());

        // Get from config how long do we need to sleep
        let sleep = get_from_config("timing","sleep_per_letter").parse::<u64>().unwrap() * answear.len() as u64;
        
        let sleep_min = get_from_config("timing","sleep_before_sending").parse::<u64>().unwrap();
        let sleep_max = get_from_config("timing","sleep_before_sending_max").parse::<u64>().unwrap();
        let sleep = sleep + rand::thread_rng().gen_range(sleep_min..sleep_max) as u64;
        timeout_future_seconds((sleep/1000).try_into().unwrap()).await;
        // Than pause the thread for that amount of time
        //thread::sleep(time::Duration::from_millis(sleep));
        
        // Finally send anwears to instaling
        let post = || -> Result<reqwest::blocking::Response, reqwest::Error> {
            Ok(hr.client.post("https://instaling.pl:443/ling2/server/actions/save_answer.php")
            .form(&map1)
            .send()?)
        };
        
        let res = match post() {
            Ok(ok) => ok,
            Err(e) => {return send_dialog_response_handler(e)}
        };

        // Parse response json
        let status;
        let parsed = json::parse({
            status = res.text_with_charset("utf-8");
            match status {
                Ok(t) => t,
                Err(e) => format!("BRUH: {:?}", e),
            }
        }.as_str()).unwrap();

        if !parsed.contains("BRUH"){
            let grade = parsed["grade"].to_string();

            // And check if we did it corectly
            if grade != "3" && grade != "0" {

                if bylo_tlumaczone{

                    // If translation was succesfull then write it to dictionary for later use
                    dictionary::write_to_dict(format!("{} $ {}",example_use,parsed["answershow"].to_string()));

                    let mut response_handler: Response = Response::new();
                    response_handler.quesion = example_use;
                    response_handler.answear = parsed["answershow"].to_string();
                    response_handler.approx = parsed["answershow"].to_string();
                    response_handler.pol_answer = polnish;
                    response_handler.succ = true;
                    response_handler

                } else{
                    let mut response_handler: Response = Response::new();
                    response_handler.quesion = example_use;
                    response_handler.answear = parsed["answershow"].to_string();
                    response_handler.approx = parsed["answershow"].to_string();
                    response_handler.pol_answer = polnish;
                    response_handler.succ = true;
                    response_handler
                }
            } else{

                // If translation was unsuccesfull then write answer that was sent in response to dictionary for later use
                dictionary::write_to_dict(format!("{} $ {}",example_use,parsed["answershow"].to_string()));

                let mut response_handler: Response = Response::new();
                response_handler.quesion = example_use;
                response_handler.answear = parsed["answershow"].to_string();
                response_handler.approx = answear;
                response_handler.pol_answer = polnish;
                response_handler.succ = false;
                response_handler
            }
        }else {
            let mut response_handler: Response = Response::new();
            response_handler.dialog_show = true;
            response_handler.dialog_title = "Error".to_string();
            response_handler.dialog_message = "Something went wrong".to_string();
            response_handler
        }
    }
}

pub fn send_dialog_handler_init(mut hs: HandlerStruct, e: reqwest::Error) -> HandlerStruct{
    hs.dialog_show = true;
    hs.dialog_title = "Error Occured".to_string();
    hs.dialog_message = e.to_string();
    hs
}
pub fn send_dialog_response_handler(e: reqwest::Error) -> Response{
    let mut res: Response = Response::new();
    res.dialog_show = true;
    res.dialog_title = "Error Occured".to_string();
    res.dialog_message = e.to_string();
    res
}