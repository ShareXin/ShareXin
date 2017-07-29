extern crate time;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate open;
use std::rc::Rc;
use std::cell::RefCell;
use gdk::enums::key;
use gtk::traits::*;
use gtk::*;
use std::*;
use service;
use Destination;

pub fn gui(service: Destination, image_bool: bool)
{
    //if gtk dont init, ends program

    match gtk::init() {
        Ok(ok) => ok,
        Err(e) => panic!("GTK could not initialize. {:?}", e),
    };

    //all theses get objects from a glade builder

    let builder = gtk::Builder::new_from_string(include_str!("sharexin.glade"));
    let window: gtk::Window = builder.get_object("window").unwrap();
    let header: gtk::HeaderBar = builder.get_object("header").unwrap();
    let text: gtk::TextView = builder.get_object("text").unwrap();
    let cancel: gtk::Button = builder.get_object("cancel").unwrap();
    let send: gtk::Button = builder.get_object("send").unwrap();
    let image: gtk::Button = builder.get_object("image").unwrap();
    let count: gtk::Label = builder.get_object("count").unwrap();

    //readying widgets
    count.set_label("0");
    window.set_title("ShareXin");
    if service.mastodon {header.set_subtitle("Mastodon");}
    else if service.twitter {header.set_subtitle("Twitter");}

    //if non-image toot/tweet, doesnt show button
    if !image_bool {image.destroy();}

    //opens image
    image.connect_clicked(move |_| {
        let mut tmp = env::temp_dir();
        tmp.push("sharexin.png");
        let temp = tmp.to_str().unwrap().clone();
        match open::that(temp) {
            Ok(ok) => ok,
            Err(e) => panic!("Could not open. {:?}", e),
        };
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    cancel.connect_clicked(|_| {gtk::main_quit();});

    let wrap_send = Rc::new(RefCell::new(send.clone()));
    let wrap_window = Rc::new(RefCell::new(window.clone()));
    let wrap_text = Rc::new(RefCell::new(text.clone()));
    let wrap_count = Rc::new(RefCell::new(count.clone()));

    {
        //the magic of button clicks
        let send = wrap_send.clone();
        let window = wrap_window.clone();
        let text = wrap_text.clone();
        send.borrow().connect_clicked(move |_| {
            //gets buffer text from TextView item
            let buffer = TextView::get_buffer(&text.borrow()).unwrap();
            let sent: Option<String> = TextBuffer::get_text(&buffer,
            &TextBuffer::get_start_iter(&buffer),
            &TextBuffer::get_end_iter(&buffer), false);
            let tweet: String = sent.unwrap();
            //creates thread, but first checks if character count is over limit
            if service.mastodon {
                if tweet.len() < 500 {
                    thread::spawn(move || {
                        glib::idle_add(move || {
                            service::thread(
                            service, image_bool.clone(), tweet.clone());
                            gtk::main_quit();
                            Continue(false)
                        });
                    });
                    window.borrow().hide();
                }
            }
            else if service.twitter {
                if tweet.len() < 140 {
                    thread::spawn(move || {
                        glib::idle_add(move || {
                            service::thread(service, image_bool.clone(), tweet.clone());
                            gtk::main_quit();
                            Continue(false)
                        });
                    });
                    window.borrow().hide();
                }
            }
        });
        }
    {
        //Control+Return also the Character Counter
        let send = wrap_send.clone();
        let window = wrap_window.clone();
        let text = wrap_text.clone();
        let count = wrap_count.clone();
        window.borrow().connect_key_press_event(move |_,key| {
            let buffer = TextView::get_buffer(&text.borrow()).unwrap();
            let sent: Option<String> = TextBuffer::get_text(&buffer,
            &TextBuffer::get_start_iter(&buffer),
            &TextBuffer::get_end_iter(&buffer), false);
            let tweet: String = sent.unwrap();
            let mut markup = String::from("<span foreground=\"#DA2E37\">");
            markup.push_str(&tweet.len().to_string());
            markup.push_str("</span>");
            if service.mastodon {
                if tweet.len() >= 500 {count.borrow().set_markup(&markup);}
                else {
                    count.borrow().set_label(&tweet.len().to_string());
                    if key.get_state().intersects(gdk::CONTROL_MASK) {
                        match key.get_keyval() {
                            key::Return => send.borrow().clicked(),
                            _ => ()
                        }
                    }
                }
            }
            else if service.twitter {
                if tweet.len() >= 140 {count.borrow().set_markup(&markup);}
                else {
                    count.borrow().set_label(&tweet.len().to_string());
                    if key.get_state().intersects(gdk::CONTROL_MASK) {
                        match key.get_keyval() {
                            key::Return => send.borrow().clicked(),
                            _ => ()
                        }
                    }
                }
            }
            Inhibit(false)
        });
    }

    {

        //Control+Return also the Character Counter
        let send = wrap_send.clone();
        let window = wrap_window.clone();
        let text = wrap_text.clone();
        let count = wrap_count.clone();
        window.borrow().connect_key_release_event(move |_,key| {
            let buffer = TextView::get_buffer(&text.borrow()).unwrap();
            let sent: Option<String> = TextBuffer::get_text(&buffer,
            &TextBuffer::get_start_iter(&buffer),
            &TextBuffer::get_end_iter(&buffer), false);
            let tweet: String = sent.unwrap();
            let mut markup = String::from("<span foreground=\"#DA2E37\">");
            markup.push_str(&tweet.len().to_string());
            markup.push_str("</span>");
            if service.mastodon {
                if tweet.len() >= 500 {count.borrow().set_markup(&markup);}
                else {
                    count.borrow().set_label(&tweet.len().to_string());
                    if key.get_state().intersects(gdk::CONTROL_MASK) {
                        match key.get_keyval() {
                            key::Return => send.borrow().clicked(),
                            _ => ()
                        }
                    }
                }
            }
            else if service.twitter {
                if tweet.len() >= 140 {count.borrow().set_markup(&markup);}
                else {
                    count.borrow().set_label(&tweet.len().to_string());
                    if key.get_state().intersects(gdk::CONTROL_MASK) {
                        match key.get_keyval() {
                            key::Return => send.borrow().clicked(),
                            _ => ()
                        }
                    }
                }
            }
            Inhibit(false)
        });

    }

    window.show_all();
    gtk::main();
}
