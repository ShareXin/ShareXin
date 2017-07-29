extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate open;
use gtk::*;
use std::*;
use std::fs::File;
use std::io::*;

pub fn gui()
{
    //if gtk dont init, ends program

    match gtk::init() {
        Ok(ok) => ok,
        Err(e) => panic!("GTK could not initialize. {:?}", e),
    };

    //all theses get objects from a glade builder

    let builder = gtk::Builder::new_from_string(include_str!("auth.glade"));
    let window: gtk::Window = builder.get_object("window").unwrap();
    let entry: gtk::Entry = builder.get_object("entry").unwrap();
    let ok: gtk::Button = builder.get_object("ok").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    ok.connect_clicked(move |_| {
        let mut file = env::var("HOME").unwrap();
        file.push_str("/.config/sharexin/pin");
        let _config = match File::create(file) {
            Ok(mut ok) => write!(ok, "{}", entry.get_text().unwrap()).unwrap(),
            _ => println!("Error."),
        };
        gtk::main_quit();
    });

    window.show_all();
    gtk::main();
}