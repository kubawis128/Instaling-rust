use gtk::Box;
use gtk::CheckButton;
use gtk::Entry;
use gtk::Image;
use gtk::ListBox;
use gtk::RadioButton;
use gtk::Widget;
use gtk::ffi::GtkBox;
use gtk::glib;
use gtk::glib::Downgrade;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button};
use crate::config_manager::*;

mod config_manager;
mod main;
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
    let start_button: Button = builder.object("start").expect("Couldn't get button7");
    let status_image: Image = builder.object("status-image").expect("Couldn't get image");

    window.set_application(Some(application));

    start_button.connect_clicked(glib::clone!(@weak window=> move |_| {
        load_config();
        if get_from_config("account","passwd") == "" {
            let builder1 = Builder::from_string(include_str!("glade.ui"));

            let login_dialog_widget: Widget = builder1.object("loginDialog").unwrap();
            login_dialog_widget.show_all();
    
            let close_button: Button = builder1.object("cancel").expect("Couldn't get cancel");
            let login_button: Button = builder1.object("login").expect("Couldn't get login");
    
    
            close_button.connect_clicked(glib::clone!(@weak login_dialog_widget => move |_| {
                login_dialog_widget.hide();
            }));
            
            login_button.connect_clicked(glib::clone!(@weak login_dialog_widget,@weak builder => move |_| {
                load_config();
                
                let username_entry: Entry = builder1.object("loginbox").expect("Couldn't get username");
                let password_entry: Entry = builder1.object("passwordbox").expect("Couldn't get password");
                let angielski_radio: RadioButton = builder.object("angielski").expect("Couldn't get angielski");
                let niemiecki_radio: RadioButton = builder.object("niemiecki").expect("Couldn't get angielski");
                let remember_checkbox: CheckButton = builder1.object("remember").expect("Couldn't get remember");
                
                // Save selected language
                let selected_lang = if angielski_radio.is_active() { "en" } else if niemiecki_radio.is_active() { "de" } else { "None" };
                set_to_config("translator", "to", Some(selected_lang));
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
        }else {
            // TODO: continue
        }

    }));
    
    /*
    status_image.set_icon_name(Some("dialog-close"));
    status_image.set_icon_name(Some("dialog-ok"));
    */
    
    // get what radiobutton is checked from "group_placeholder" group
    



    window.show_all();
}