use gtk::*;
use gtk::prelude::*;
use crate::config_manager::*;
use crate::handler::*;
mod handler;
mod dictionary;
mod config_manager;
mod translator_patched;
use crate::glib::MainContext;
use crate::glib::clone;

fn main() {
    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    // Create builder and application window
    let glade_src = include_str!("glade.ui");
    let builder = Builder::from_string(glade_src);
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    
    // Create varibles for buttons clickers and all black magic
    let login_button: Button = builder.object("main_login").expect("Couldn't get login button");
    let start_button: Button = builder.object("main_start").expect("Couldn't get start button");
    let status_image: Image = builder.object("status-image").expect("Couldn't get image");

    window.set_application(Some(application));

    login_button.connect_clicked(glib::clone!(@weak window=> move |_| {
        load_config();
        let builder1 = Builder::from_string(include_str!("glade.ui"));
        let login_dialog_widget: Widget = builder1.object("loginDialog").unwrap();
        login_dialog_widget.show_all();

        let close_button: Button = builder1.object("cancel").expect("Couldn't get cancel");
        let login_button: Button = builder1.object("login").expect("Couldn't get login");


        close_button.connect_clicked(glib::clone!(@weak login_dialog_widget => move |_| {
            login_dialog_widget.hide();
        }));
        
        login_button.connect_clicked(glib::clone!(@weak login_dialog_widget => move |_| {
            load_config();
            
            let username_entry: Entry = builder1.object("loginbox").expect("Couldn't get username");
            let password_entry: Entry = builder1.object("passwordbox").expect("Couldn't get password");
            let remember_checkbox: CheckButton = builder1.object("remember").expect("Couldn't get remember");

            // Save user login info
            set_to_config("account", "login", Some(username_entry.text().as_str()));
            if remember_checkbox.is_active() {
                config_manager::set_to_config("account", "remember", Some("true"));
                config_manager::set_to_config("account", "passwd", Some(password_entry.text().as_str()));
            }else {
                config_manager::set_to_config("account", "remember", Some("false"));
                config_manager::set_to_config("account", "passwd", Some(password_entry.text().as_str()));
            }
            login_dialog_widget.hide();
        }));
    }));

    start_button.connect_clicked(glib::clone!(@weak window,@weak start_button => move |_| {
        start_button.set_sensitive(false);
        login_button.set_sensitive(false);
        let angielski_radio: RadioButton = builder.object("angielski").expect("Couldn't get angielski");
        let niemiecki_radio: RadioButton = builder.object("niemiecki").expect("Couldn't get angielski");
        
        // Save selected language
        load_config();
        let selected_lang = if angielski_radio.is_active() { "en" } else if niemiecki_radio.is_active() { "de" } else { "None" };
        set_to_config("translator", "to", Some(selected_lang));

        
        let hr = handler_init();
        
        let quesion: Label = builder.object("question").expect("Couldn't get quesion");
        let translation: Label = builder.object("translation").expect("Couldn't get translation");
        let answear: Label = builder.object("answer").expect("Couldn't get answer");
        let progress_label: Label = builder.object("progress").expect("Couldn't get progress_label");
        let progress_bar: ProgressBar = builder.object("progressprogressbar").expect("Couldn't get progress_bar");

        let main_context = MainContext::default();
        main_context.spawn_local(clone!(@weak quesion,@weak status_image,@weak login_button => async move {
            loop{
                //run this async
                let res: Response = loop_de_loop(hr.clone()).await;
                            
                if res.dialog_show{
                    println!("Done!");
                    start_button.set_sensitive(true);
                    login_button.set_sensitive(true);
                    load_config();
                    if get_from_config("account","remember") == "false" {
                        set_to_config("account","passwd",Some(""));
                    }
                    glib::MainContext::default().spawn_local(dialog(window,res.dialog_message,res.dialog_title));
                    break
                }else if res.ignore{

                }else{
                    quesion.set_text(&res.quesion);
                    translation.set_text(&res.pol_answer);
                    answear.set_text(&res.answear);
                    println!("{}", res.succ);
                    if res.succ {
                        status_image.set_icon_name(Some("dialog-ok"));
                        let mut e: String = progress_label.text().parse().unwrap();
                        e = e.split("/").nth(0).unwrap().to_string();
                        e.parse::<f64>().unwrap();
                        progress_label.set_text(&format!("{0}/20",e.parse::<i64>().unwrap() + 1));
                        progress_bar.set_fraction((e.parse::<f64>().unwrap()+1.0)/20.0)
                    }else {
                        status_image.set_icon_name(Some("dialog-close"));
                    }
                }
            }
            
        }));
        
    }));
    /*
    status_image.set_icon_name(Some("dialog-close"));
    status_image.set_icon_name(Some("dialog-ok"));
    */
    window.show_all();
}

async fn dialog<W: IsA<gtk::Window>>(window: W,text: String,title: String) {
    let question_dialog = gtk::MessageDialog::builder()
        .transient_for(&window)
        .modal(true)
        .buttons(gtk::ButtonsType::Ok)
        .text(&text)
        .title(&title)
        .build();
    question_dialog.close();
    question_dialog.hide();
}