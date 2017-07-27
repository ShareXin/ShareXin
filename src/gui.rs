extern crate time;
extern crate gtk;
extern crate glib;
extern crate gdk;
use std::rc::Rc;
use std::cell::RefCell;
use gdk::enums::key;
use gtk::traits::*;
use gtk::*;
use std::*;
use send;

pub fn gui(mort: bool, morti: bool)
{
    if gtk::init().is_err() {
        println!("GTK did not init.");
        return;
    }

    let builder = gtk::Builder::new_from_string(include_str!("sharexin.glade"));
    let window: gtk::Window = builder.get_object("window").unwrap();
    let header: gtk::HeaderBar = builder.get_object("header").unwrap();
    let text: gtk::TextView = builder.get_object("text").unwrap();
    let cancel: gtk::Button = builder.get_object("cancel").unwrap();
    let send: gtk::Button = builder.get_object("send").unwrap();
    window.set_title("ShareXin");
    if mort {
        header.set_subtitle("Mastodon");
    }
    else {
        header.set_subtitle("Twitter");
    }

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    cancel.connect_clicked(|_| {
        gtk::main_quit();
    });

    let wrap_send = Rc::new(RefCell::new(send));
    let wrap_widow = Rc::new(RefCell::new(window.clone()));

    {
        let send = wrap_send.clone();
        let window = wrap_widow.clone();
        send.borrow().connect_clicked(move |_| {
            let buffer = TextView::get_buffer(&text).unwrap();
            let sent: Option<String> = TextBuffer::get_text(&buffer,
            &TextBuffer::get_start_iter(&buffer),
            &TextBuffer::get_end_iter(&buffer), false);
            let tweet: String = sent.unwrap();
            if mort {
                if morti {
                    thread::spawn(move || {
                        glib::idle_add(move || {
                            send::toot_img(tweet.clone());
                            gtk::main_quit();
                            Continue(false)
                        });
                     });
                }
                else if !tweet.is_empty() {
                    thread::spawn(move || {
                        glib::idle_add(move || {
                            send::toot(tweet.clone());
                            gtk::main_quit();
                            Continue(false)
                        });
                     });
                }
                else {
                    send::notification_3(mort);
                    gtk::main_quit();
                }
            }
            else {
                if morti {
                    thread::spawn(move || {
                        glib::idle_add(move || {
                            send::twitter_img(tweet.clone());
                            gtk::main_quit();
                            Continue(false)
                        });
                     });
                }
                else if !tweet.is_empty() {
                    thread::spawn(move || {
                        glib::idle_add(move || {
                            send::twitter(tweet.clone());
                            gtk::main_quit();
                            Continue(false)
                        });
                     });
                }
                else {
                    send::notification_3(mort);
                    gtk::main_quit();
                }
            }
            window.borrow().hide();
        });
        }
    {
        let send = wrap_send.clone();
        let window = wrap_widow.clone();
        window.borrow().connect_key_press_event(move |_,key| {
            if key.get_state().intersects(gdk::CONTROL_MASK) {
                match key.get_keyval() {
                    key::Return => send.borrow().clicked(),
                    _ => ()
                }
            }
            Inhibit(false)
        });
    }

    window.show_all();
    gtk::main();
}
