extern crate curl;
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate notify_rust;
extern crate open;
extern crate time;

#[macro_use]
extern crate clap;
extern crate clipboard;
extern crate imgur as Imgur;
extern crate yaml_rust;
mod cmd;
mod dialog;
mod error;
mod image;
mod imgur;
mod language;
mod mastodon;
mod notification;
mod save;
mod screenshot;
mod twitter;
mod upgrade;

pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub static SHAREXIN: &'static str = "https://github.com/ShareXin/ShareXin";

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

fn main() {
    cmd::cmd();
}
