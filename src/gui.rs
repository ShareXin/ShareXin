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
use mort;

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
    let image: gtk::Button = builder.get_object("image").unwrap();
    let count: gtk::Label = builder.get_object("count").unwrap();
    count.set_label("0");
    window.set_title("ShareXin");
    if mort {
        header.set_subtitle("Mastodon");
    }
    else {
        header.set_subtitle("Twitter");
    }

    if !morti {
        image.destroy();
    }

    image.connect_clicked(move |_| {
        let mut tmp = env::temp_dir();
        tmp.push("sharexin.png");
        let temp = tmp.to_str().unwrap().clone();
        let _ = open::that(temp);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    cancel.connect_clicked(|_| {
        gtk::main_quit();
    });

    let wrap_send = Rc::new(RefCell::new(send.clone()));
    let wrap_widow = Rc::new(RefCell::new(window.clone()));
    let wrap_text = Rc::new(RefCell::new(text.clone()));

    {
        let send = wrap_send.clone();
        let window = wrap_widow.clone();
        let text = wrap_text.clone();
        send.borrow().connect_clicked(move |_| {
            let buffer = TextView::get_buffer(&text.borrow()).unwrap();
            let sent: Option<String> = TextBuffer::get_text(&buffer,
            &TextBuffer::get_start_iter(&buffer),
            &TextBuffer::get_end_iter(&buffer), false);
            let tweet: String = sent.unwrap();
            thread::spawn(move || {
                glib::idle_add(move || {
                    mort::thread(mort.clone(), morti.clone(), tweet.clone());
                    gtk::main_quit();
                    Continue(false)
                });
            });
            window.borrow().hide();
        });
        }
    {
        let send = wrap_send.clone();
        let window = wrap_widow.clone();
        let text = wrap_text.clone();
        window.borrow().connect_key_press_event(move |_,key| {
            if key.get_state().intersects(gdk::CONTROL_MASK) {
                match key.get_keyval() {
                    key::Return => send.borrow().clicked(),
                    _ => ()
                }
            }
            let buffer = TextView::get_buffer(&text.borrow()).unwrap();
            let sent: Option<String> = TextBuffer::get_text(&buffer,
            &TextBuffer::get_start_iter(&buffer),
            &TextBuffer::get_end_iter(&buffer), false);
            let tweet: String = sent.unwrap();
            count.set_label(&tweet.len().to_string());
            if mort {
                if tweet.len() == 500 {
                    text.borrow().set_editable(false);
                }
            }
            else if !mort {
                if tweet.len() == 140 {
                    text.borrow().set_editable(false);
                }
            }
            Inhibit(false)
        });
    }

    window.show_all();
    gtk::main();
}
