#![allow(dead_code)]
#![allow(unused_imports)]

extern crate notify_rust;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate open;
extern crate time;
extern crate curl;
extern crate yaml_rust;
extern crate clipboard;
extern crate systray;
extern crate imgur as Imgur;
extern crate sharexin;
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
mod tray;
mod screenshot;
use sharexin::Destination;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static SHAREXIN: &'static str = "https://crates.io/crates/sharexin";

fn main() {
    cmd::cmd();
}
