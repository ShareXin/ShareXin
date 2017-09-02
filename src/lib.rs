#![allow(unused_imports)]

extern crate notify_rust;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate open;
extern crate time;
extern crate curl;
#[macro_use]
extern crate clap;
extern crate yaml_rust;
extern crate clipboard;
extern crate systray;
extern crate imgur as Imgur;
pub mod notification;
pub mod twitter;
pub mod mastodon;
pub mod image;
pub mod dialog;
pub mod imgur;
pub mod error;
pub mod upgrade;
pub mod language;
pub mod save;
pub mod desktop;
pub mod screenshot;
use dialog::dialog;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static SHAREXIN: &'static str = "https://crates.io/crates/sharexin";

#[derive(Debug, Clone, Copy)]
pub struct Destination {
    pub twitter: bool,
    pub mastodon: bool,
    pub imgur: bool,
}

impl Destination {
    pub fn new(id: usize) -> Destination {
        if id == 0 {
            Destination {
                twitter: false,
                mastodon: true,
                imgur: false,
            }
        } else if id == 1 {
            Destination {
                twitter: true,
                mastodon: false,
                imgur: false,
            }
        } else if id == 2 {
            Destination {
                twitter: false,
                mastodon: false,
                imgur: true,
            }
        } else {
            Destination {
                twitter: false,
                mastodon: false,
                imgur: false,
            }
        }
    }
    pub fn name(self) -> String {
        if self.mastodon {
            "Mastodon".to_owned()
        } else if self.twitter {
            "Twitter".to_owned()
        } else if self.imgur {
            "Imgur".to_owned()
        } else {
            "".to_owned()
        }
    }
}

pub fn tweet() {
    dialog::dialog(Destination::new(1), false);
}

pub fn toot() {
    dialog::dialog(Destination::new(0), false);
}

pub fn tweet_full() {
    image::image(2);
    dialog(Destination::new(1), true);
}

pub fn tweet_window() {
    image::image(1);
    dialog(Destination::new(1), true);
}

pub fn tweet_area() {
    image::image(0);
    dialog(Destination::new(1), true);
}

pub fn toot_full() {
    image::image(2);
    dialog(Destination::new(0), true);
}

pub fn toot_window() {
    image::image(1);
    dialog(Destination::new(0), true);
}

pub fn toot_area() {
    image::image(0);
    dialog(Destination::new(0), true);
}

pub fn imgur_full() {
    image::image(2);
    imgur::send();
}

pub fn imgur_window() {
    image::image(1);
    imgur::send();
}

pub fn imgur_area() {
    image::image(0);
    imgur::send();
}

pub fn tweet_file(filed: String) {
    save::file(filed);
    dialog(Destination::new(1), true);
}

pub fn toot_file(filed: String) {
    save::file(filed);
    dialog(Destination::new(0), true);
}

pub fn imgur_file(filed: String) {
    save::file(filed);
    imgur::send();
}
