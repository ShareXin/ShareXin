#[macro_use]
extern crate clap;

extern crate clipboard;
extern crate curl;
extern crate egg_mode;
extern crate egg_mode_text;
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate imgur as Imgur;
extern crate mammut;
extern crate notify_rust;
extern crate open;
extern crate screenshot_rs;
extern crate time;
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
mod twitter;
mod upgrade;

pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub static SHAREXIN: &'static str = "https://github.com/ShareXin/ShareXin";

#[derive(Copy, Clone)]
pub enum ServiceKind {
    Twitter,
    Mastodon,
    Imgur,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MessageKind {
    Image,
    Text,
}

fn main() {
    cmd::cmd();
}
