extern crate libnotify;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate open;
use std::*;
mod notification;
mod send;
mod file;
mod help;
mod gui;
mod service;
use gui::gui;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static SHAREXIN: &'static str = "https://crates.io/crates/sharexin";

fn main()
{
    //gets command args
    let args: Vec<_> = env::args().collect();
    //if at least one arg is passed (the command itself counts?? wtf)
    if args.len() >= 2 {
        //check the first arg
        match args[1].as_ref() {
            "-V" | "--version" | "version" => println!("sharexin {}", VERSION),
            "-U" | "--upgrade" | "upgrade" => help::upgrade(),
            "toot" => {
                if args.len() == 3 {
                    //if first arg is for mastodon, check if its an acceptable arg
                    match args[2].as_ref() {
                        "area" | "-a" => {
                            file::image(String::from("-s"));
                            gui(true, true);
                        },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                            gui(true, true);
                        },
                        "full" => {
                            file::image(String::new());
                            gui(true, true);
                        },
                        _ => help::help()
                    }
                }
                //if its only toot, then no image will be taken
                else {
                    gui(true, false);
                }
            },
            //if first arg is twitter
            "tweet" => {
                if args.len() == 3 {
                    //check any more args
                    match args[2].as_ref() {
                        "area" | "-a" => {
                            file::image(String::from("-s"));
                            gui(false, true);
                        },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                            gui(false, true);
                        },
                        "full" => {
                            file::image(String::new());
                            gui(false, true);
                        },
                        _ => help::help()
                    }
                }
                //or just tweet
                else {
                    gui(false, false);
                }
            },
            //if first arg is file
            "file" => {
                if args.len() == 3 {
                    match args[2].as_ref() {
                        //check for args
                        "area" | "-a" => file::image(String::from("-s")),
                        "window" | "-w" => file::image(String::from("-i")),
                        "full" => file::image(String::new()),
                        _ => help::help()
                    }
                }
            },
            //or else give them help, you cant save nothing.
            _ => help::help()
        }
    }
    //if no args at all
    else {
        help::help();
    }
}
