extern crate libnotify;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate open;
extern crate time;
extern crate curl;
extern crate imgur as Imgur;
mod notification;
mod twitter;
mod mastodon;
mod file;
mod help;
mod gui;
mod imgur;
mod cmd;
mod language;
use cmd::cmd;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static SHAREXIN: &'static str = "https://crates.io/crates/sharexin";

#[derive(Debug, Clone, Copy)]
pub struct Destination {
    twitter: bool,
    mastodon: bool,
    imgur: bool,
}

impl Destination {
    pub fn new(id: u32) -> Destination {
        if id == 0 {
            Destination { twitter: false, mastodon: true, imgur: false }
        }
        else if id == 1 {
            Destination { twitter: true, mastodon: false, imgur: false }
        }
        else if id == 2 {
            Destination { twitter: false, mastodon: false, imgur: true }
        }
        else {
            Destination { twitter: false, mastodon: false, imgur: false }
        }
    }
}

fn main()
{
    cmd();
}
