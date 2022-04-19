use gtk::CheckButton;
use gtk::Entry;
use gtk::Widget;
use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button};

fn main() {
    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default());
    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("glade.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let start_button: Button = builder.object("start").expect("Couldn't get button7");
    // add function to startButton
    start_button.connect_clicked(glib::clone!(@weak window => move |_| {

        let builder = Builder::from_string(include_str!("glade.ui"));
        let login_dialog_widget: Widget = builder.object("loginDialog").unwrap();

        login_dialog_widget.show_all();

        let close_button: Button = builder.object("cancel").expect("Couldn't get cancel");
        let login_button: Button = builder.object("login").expect("Couldn't get login");

        close_button.connect_clicked(glib::clone!(@weak login_dialog_widget => move |_| {
            login_dialog_widget.hide();
        }));
        
        login_button.connect_clicked(glib::clone!(@weak login_dialog_widget => move |_| {
            
            // print entry values
            let username_entry: Entry = builder.object("loginbox").expect("Couldn't get username");
            let password_entry: Entry = builder.object("passwordbox").expect("Couldn't get password");
            println!("Username: {}", username_entry.text());
            println!("Password: {}", password_entry.text());

            // print if checkbox with id "remember" is active 
            let remember_checkbox: CheckButton = builder.object("remember").expect("Couldn't get remember");
            println!("Remember: {:?}", remember_checkbox.is_active());
            login_dialog_widget.hide();
        }));

    }));

    
    window.show_all();
}