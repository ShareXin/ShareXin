use error;
use gdk;
use gdk::enums::key;
use glib;
use gtk;
use gtk::*;
use image;
use mastodon;
use notification;
use std::borrow::Borrow;
use std::thread;
use twitter;
use ServiceKind;
use MessageKind;

pub fn dialog(service: ServiceKind, message: MessageKind) {
    match gtk::init() {
        Ok(ok) => ok,
        Err(_) => eprintln!("{}", error::message(24)),
    };

    // objects from glade file
    let builder = gtk::Builder::new_from_string(include_str!("sharexin.glade"));
    let window: gtk::Window = builder.get_object("window").unwrap();
    let header: gtk::HeaderBar = builder.get_object("header").unwrap();
    let text: gtk::TextView = builder.get_object("text").unwrap();
    let cancel: gtk::Button = builder.get_object("cancel").unwrap();
    let send: gtk::Button = builder.get_object("send").unwrap();
    let image: gtk::Button = builder.get_object("image").unwrap();
    let count: gtk::Label = builder.get_object("count").unwrap();

    // setting widgets
    window.set_title("ShareXin");
    window.set_hide_titlebar_when_maximized(false);
    window.set_keep_above(true);

    match service {
        ServiceKind::Twitter => {
            match message {
                MessageKind::Image => count.set_label("257"),
                MessageKind::Text => count.set_label("280")
            };
        },
        ServiceKind::Mastodon => {
            header.set_subtitle("Mastodon");
            count.set_label("500");
        },
        ServiceKind::Imgur => panic!("Not possible")
    }

    /*// if non-image toot/tweet, doesnt show file button
    if message == MessageKind::Image {
        image.destroy();
    }

    image.connect_clicked(move |_| {
        image::open_temp();
        return;
    });

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        if image_bool {
            image::delete_temp();
        }
        Inhibit(false)
    });

    cancel.connect_clicked(move |_| {
        gtk::main_quit();
        if image_bool {
            image::delete_temp();
        }
        Inhibit(false);
    });

    // rust security bypass
    let send_bypass = send.clone();
    let window_bypass = window.clone();
    let text_bypass = text.clone();

    // checks textview and sends message to destination/service
    send_bypass.borrow().connect_clicked(move |_| {
        // gets buffer text from TextView item
        let buffer = TextView::get_buffer(&text_bypass.borrow()).unwrap();

        let sent: Option<String> = TextBuffer::get_text(
            &buffer,
            &TextBuffer::get_start_iter(&buffer),
            &TextBuffer::get_end_iter(&buffer),
            false,
        );
        let status: String = sent.unwrap();
        // checks if character count is over limit, then creates thread for sending and closes
        match service {
            ServiceKind::Twitter => {
                match message {
                    MessageKind::Image => {
                        thread::spawn(move || {
                            glib::idle_add(move || {
                                twitter::image(status.clone());
                                gtk::main_quit();
                                Continue(false)
                            });
                        });
                    window_bypass.borrow().hide();
                    },
                    MessageKind::Text => {
                        thread::spawn(move || {
                            glib::idle_add(move || {
                              // if its not empty, send
                               if !status.is_empty() {
                                   twitter::tweet(status.clone());
                                   gtk::main_quit();
                                   Continue(false)
                               }
                            });
                        });
                        window_bypass.borrow().hide();
                    }
                }
            },
            ServiceKind::Mastodon => {
            },
            ServiceKind::Imgur => {
            }
        }
        /*if service.mastodon {
            if message.len() <= 500 {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        // image_bool is true for yes image, false for no image
                        if image_bool {
                            mastodon::image(message.clone());
                        }
                        // if false, then if its not empty, send
                        else if !message.is_empty() {
                            mastodon::toot(message.clone());
                        }
                        // if empty, cancel and notify
                        else {
                            notification::empty(service);
                        }

                        gtk::main_quit();
                        Continue(false)
                    });
                });
                window_bypass.borrow().hide();
            }
        } else if service.twitter {
            if message.len() <= 280 && !image_bool {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        // if its not empty, send
                        if !message.is_empty() {
                            twitter::tweet(message.clone());
                        }
                        // if empty, cancel and notify
                        else {
                            notification::empty(service);
                        }

                        gtk::main_quit();
                        Continue(false)
                    });
                });
                window_bypass.borrow().hide();
            } else if message.len() <= 257 && image_bool {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        twitter::image(message.clone());
                        gtk::main_quit();
                        Continue(false)
                    });
                });
                window_bypass.borrow().hide();
            }
        }*/
    });

    // control+return sends message
    // if twitter, 280 char limit, if mastodon 500 CHAR LIMIT
    let send_bypass = send.clone();
    let window_bypass = window.clone();
    let text_bypass = text.clone();
    let count_bypass = count.clone();
    window_bypass
        .borrow()
        .connect_key_press_event(move |_, key| {
            let buffer = TextView::get_buffer(&text_bypass.borrow()).unwrap();
            let sent: Option<String> = TextBuffer::get_text(
                &buffer,
                &TextBuffer::get_start_iter(&buffer),
                &TextBuffer::get_end_iter(&buffer),
                false,
            );
            let status: String = sent.unwrap();

            let status_len = char_count(service.clone(), status.clone(), message.clone());

            // uses markdown to set color
            let mut limit = String::from("<span foreground=\"#DA2E37\">");
            limit.push_str(&status_len.to_string());
            limit.push_str("</span>");
            let mut hit = String::from("<span foreground=\"#e4e543\">");
            hit.push_str(&status_len.to_string());
            hit.push_str("</span>");
            /*if service.mastodon {
                if message_len == 0 {
                    count_bypass.borrow().set_markup(&hit);
                } else if message_len < 0 {
                    count_bypass.borrow().set_markup(&limit);
                } else {
                    count_bypass.borrow().set_label(&message_len.to_string());
                    if key.get_state().intersects(gdk::ModifierType::CONTROL_MASK) {
                        match key.get_keyval() {
                            key::Return => send_bypass.borrow().clicked(),
                            _ => (),
                        }
                    }
                }
            } else if service.twitter {
                if message_len == 0 {
                    count_bypass.borrow().set_markup(&hit);
                } else if message_len < 0 {
                    count_bypass.borrow().set_markup(&limit);
                } else {
                    count_bypass.borrow().set_label(&message_len.to_string());
                    if key.get_state().intersects(gdk::ModifierType::CONTROL_MASK) {
                        match key.get_keyval() {
                            key::Return => send_bypass.borrow().clicked(),
                            _ => (),
                        }
                    }
                }
            }*/
            Inhibit(false)
        });

    // same as connect_key_press but when any key is released
    let send_bypass = send.clone();
    let window_bypass = window.clone();
    let text_bypass = text.clone();
    let count_bypass = count.clone();
    window_bypass
        .borrow()
        .connect_key_release_event(move |_, key| {
            let buffer = TextView::get_buffer(&text_bypass.borrow()).unwrap();
            let sent: Option<String> = TextBuffer::get_text(
                &buffer,
                &TextBuffer::get_start_iter(&buffer),
                &TextBuffer::get_end_iter(&buffer),
                false,
            );
            let status: String = sent.unwrap();

            let status_len = char_count(service.clone(), status.clone(), message.clone());

            // uses markdown to set color
            let mut limit = String::from("<span foreground=\"#DA2E37\">");
            limit.push_str(&status_len.to_string());
            limit.push_str("</span>");
            let mut hit = String::from("<span foreground=\"#e4e543\">");
            hit.push_str(&status_len.to_string());
            hit.push_str("</span>");
            /*if service.mastodon {
                if message_len == 0 {
                    count_bypass.borrow().set_markup(&hit);
                } else if message_len < 0 {
                    count_bypass.borrow().set_markup(&limit);
                } else {
                    count_bypass.borrow().set_label(&message_len.to_string());
                    if key.get_state().intersects(gdk::ModifierType::CONTROL_MASK) {
                        match key.get_keyval() {
                            key::Return => send_bypass.borrow().clicked(),
                            _ => (),
                        }
                    }
                }
            } else if service.twitter {
                if message_len == 0 {
                    count_bypass.borrow().set_markup(&hit);
                } else if message_len < 0 {
                    count_bypass.borrow().set_markup(&limit);
                } else {
                    count_bypass.borrow().set_label(&message_len.to_string());
                    if key.get_state().intersects(gdk::ModifierType::CONTROL_MASK) {
                        match key.get_keyval() {
                            key::Return => send_bypass.borrow().clicked(),
                            _ => (),
                        }
                    }
                }
            }*/
            Inhibit(false)
        });*/

    window.show_all();
    gtk::main();
}

fn char_count(service: ServiceKind, message: MessageKind, status: String) -> isize {
    match service {
        ServiceKind::Twitter => {
            match message {
                MessageKind::Image => return 257 - status.len() as isize,
                MessageKind::Text => return 280 - status.len() as isize
            }
        },
        ServiceKind::Mastodon => return 500 - status.len() as isize,
        ServiceKind::Imgur => return 0 as isize
    }
}
