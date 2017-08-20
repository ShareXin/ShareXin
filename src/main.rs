extern crate notify_rust;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate open;
extern crate time;
extern crate curl;
extern crate yaml_rust;
extern crate clipboard;
extern crate imgur as Imgur;
mod notification;
mod twitter;
mod mastodon;
mod image;
mod dialog;
mod imgur;
mod cmd;
mod error;
mod upgrade;
mod language;
mod save;
mod desktop;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static SHAREXIN: &'static str = "https://crates.io/crates/sharexin";

#[derive(Debug, Clone, Copy)]
pub struct Destination {
    twitter: bool,
    mastodon: bool,
    imgur: bool,
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

    let user = match ::std::env::var("USER") {
        Ok(ok) => ok,
        Err(_) => String::new(),
    };
    if user != "root" {
        cmd::cmd();
    } else {
        error::fatal();
    }

}
