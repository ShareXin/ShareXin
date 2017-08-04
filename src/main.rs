/*
    Flow of program:
        main.rs only defines a Destination struct, and opens cmd.rs
        cmd.rs checks user cmd args and launches the appropriate functions
        image-based commands launch file.rs, fn:open being if you're using a pre-made file
            fn:image being taking a screenshot with an appropriate screenshot application
            fn:image then calls fn:save, to save the file to a location in your Pictures folder
        when gui.rs is called, it decides whether to call itself Mastodon or Twitter
            depending on the Destination struct,
            and when the user sends their message
            it launches the appropriate service function
        when imgur.rs is called, the file is sent to imgur
            and a notification is displayed and the link is opened
        notification.rs first identifies which type of notification
            it may be {file saved, sent to twitter, etc..}
            but first calls language to get the string itself
                in various different languages, depending on $LANG
        help.rs is what cmd.rs calls if the user doesn't know what they're doing
            and it too has many languages
*/

#[cfg(target_os = "linux")]
extern crate notify_rust;

#[cfg(target_os = "macos")]
extern crate mac_notification_sys;

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
