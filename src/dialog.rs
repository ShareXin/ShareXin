use egg_mode_text;
use error;
use gdk;
use gdk::enums::key;
use glib;
use gtk;
use gtk::*;
use image;
use mastodon;
use std::borrow::Borrow;
use std::thread;
use twitter;
use MessageKind;
use ServiceKind;

const URL_COUNT: i32 = 23;
const TWITTER_COUNT: i32 = 280;
const MASTODON_COUNT: i32 = 500;
const TWITTER_IMAGE_COUNT: i32 = TWITTER_COUNT - URL_COUNT;

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

    match service {
        ServiceKind::Twitter => {
            header.set_subtitle("Twitter");
            match message {
                MessageKind::Image => count.set_label(&TWITTER_IMAGE_COUNT.to_string()),
                MessageKind::Text => count.set_label(&TWITTER_COUNT.to_string()),
            };
        }
        ServiceKind::Mastodon => {
            header.set_subtitle("Mastodon");
            count.set_label(&MASTODON_COUNT.to_string());
        }
        ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
    }

    // if non-image toot/tweet, doesnt show file button
    if message != MessageKind::Image {
        image.destroy();
        send.set_sensitive(false);
    }

    image.connect_clicked(move |_| {
        image::open_temp();
        return;
    });

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        if message == MessageKind::Image {
            image::delete_temp();
        }
        Inhibit(false)
    });

    cancel.connect_clicked(move |_| {
        gtk::main_quit();
        if message == MessageKind::Image {
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
            ServiceKind::Twitter => match message {
                MessageKind::Image => {
                    thread::spawn(move || {
                        glib::idle_add(move || {
                            twitter::image(status.clone());
                            gtk::main_quit();
                            Continue(false)
                        });
                    });
                    window_bypass.borrow().hide();
                }
                MessageKind::Text => {
                    if !status.is_empty() {
                        thread::spawn(move || {
                            glib::idle_add(move || {
                                twitter::tweet(status.clone());
                                gtk::main_quit();

                                Continue(false)
                            });
                        });
                        window_bypass.borrow().hide();
                    }
                }
            },
            ServiceKind::Mastodon => match message {
                MessageKind::Image => {
                    thread::spawn(move || {
                        glib::idle_add(move || {
                            mastodon::image(status.clone());
                            gtk::main_quit();
                            Continue(false)
                        });
                    });
                    window_bypass.borrow().hide();
                }
                MessageKind::Text => {
                    if !status.is_empty() {
                        thread::spawn(move || {
                            glib::idle_add(move || {
                                mastodon::toot(status.clone());
                                gtk::main_quit();

                                Continue(false)
                            });
                        });
                        window_bypass.borrow().hide();
                    }
                }
            },
            ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
        }
    });

    // if twitter, 280 char limit, if mastodon 500 CHAR LIMIT
    let send_bypass = send.clone();
    let window_bypass = window.clone();
    let text_bypass = text.clone();
    let count_bypass = count.clone();

    window_bypass.borrow().connect_key_press_event(move |_, _| {
        key_event(
            send_bypass.clone(),
            text_bypass.clone(),
            count_bypass.clone(),
            message.clone(),
            service.clone(),
        );
        Inhibit(false)
    });

    let send_bypass = send.clone();
    let window_bypass = window.clone();
    let text_bypass = text.clone();
    let count_bypass = count.clone();

    window_bypass
        .borrow()
        .connect_key_release_event(move |_, _| {
            key_event(
                send_bypass.clone(),
                text_bypass.clone(),
                count_bypass.clone(),
                message.clone(),
                service.clone(),
            );
            Inhibit(false)
        });

    fn key_event(
        send_bypass: gtk::Button,
        text_bypass: gtk::TextView,
        count_bypass: gtk::Label,
        message: MessageKind,
        service: ServiceKind,
    ) {
        let buffer = TextView::get_buffer(&text_bypass).unwrap();
        let sent: Option<String> = TextBuffer::get_text(
            &buffer,
            &TextBuffer::get_start_iter(&buffer),
            &TextBuffer::get_end_iter(&buffer),
            false,
        );
        let status: String = sent.unwrap();

        let status_count = char_count(service, status, message);

        // uses markdown to set color
        let mut limit = format!("<span foreground=\"#EE0456\">");
        limit.push_str(&status_count.to_string());
        limit.push_str("</span>");
        let mut hit = format!("<span foreground=\"#ECA60B\">");
        hit.push_str(&status_count.to_string());
        hit.push_str("</span>");

        match message {
            MessageKind::Image => match service {
                ServiceKind::Twitter => {
                    if status_count <= 20 && status_count >= 0 {
                        count_bypass.set_markup(&hit);
                    } else if status_count < 0 {
                        count_bypass.set_markup(&limit);
                    } else {
                        count_bypass.set_label(&status_count.to_string());
                    }
                }
                ServiceKind::Mastodon => {
                    if status_count < 0 {
                        count_bypass.set_markup(&limit);
                    } else {
                        count_bypass.set_label(&status_count.to_string());
                    }
                }
                ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
            },
            MessageKind::Text => match service {
                ServiceKind::Twitter => {
                    if status_count >= TWITTER_COUNT || status_count < 0 {
                        send_bypass.set_sensitive(false);
                    } else {
                        send_bypass.set_sensitive(true);
                    }
                    if status_count <= 20 && status_count >= 0 {
                        count_bypass.set_markup(&hit);
                    } else if status_count < 0 {
                        count_bypass.set_markup(&limit);
                    } else {
                        count_bypass.set_label(&status_count.to_string());
                    }
                }
                ServiceKind::Mastodon => {
                    if status_count >= MASTODON_COUNT || status_count < 0 {
                        send_bypass.set_sensitive(false);
                    } else {
                        send_bypass.set_sensitive(true);
                    }
                    if status_count < 0 {
                        count_bypass.set_markup(&limit);
                    } else {
                        count_bypass.set_label(&status_count.to_string());
                    }
                }
                ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
            },
        }
    }

    // control+return sends message
    let send_bypass = send.clone();
    let window_bypass = window.clone();

    window_bypass
        .borrow()
        .connect_key_press_event(move |_, key| {
            if key.get_state().intersects(gdk::ModifierType::CONTROL_MASK) {
                match key.get_keyval() {
                    key::Return => {
                        if send_bypass.get_sensitive() {
                            send_bypass.clicked();
                        }
                    }
                    _ => (),
                }
            }
            Inhibit(false)
        });

    window.show_all();
    gtk::main();
}

fn char_count(service: ServiceKind, status: String, message: MessageKind) -> i32 {
    let remaining = match service {
        ServiceKind::Twitter => match message {
            MessageKind::Image => {
                TWITTER_IMAGE_COUNT
                    - egg_mode_text::character_count(&status, URL_COUNT, URL_COUNT) as i32
            }
            MessageKind::Text => {
                TWITTER_COUNT - egg_mode_text::character_count(&status, URL_COUNT, URL_COUNT) as i32
            }
        },
        ServiceKind::Mastodon => {
            MASTODON_COUNT - egg_mode_text::character_count(&status, URL_COUNT, URL_COUNT) as i32
        }
        ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
    };
    return remaining;
}
