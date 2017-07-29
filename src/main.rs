#![allow(dead_code)]
#![allow(unused_imports)]

extern crate libnotify;
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate open;
use std::*;
mod notification;
mod twitter;
mod mastodon;
mod file;
mod help;
mod gui;
mod imgur;
mod auth;
mod service;
mod language;
use gui::gui;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static SHAREXIN: &'static str = "https://crates.io/crates/sharexin";

#[derive(Debug, Clone, Copy)]
pub struct Destination {
    twitter: bool,
    mastodon: bool,
    imgur: bool,
}

fn main()
{
    let twitter = Destination { twitter: true, mastodon: false, imgur: false };
    let mastodon = Destination { twitter: false, mastodon: true, imgur: false };
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
                            gui(mastodon, true);
                        },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                            gui(mastodon, true);
                        },
                        "full" => {
                            file::image(String::new());
                            gui(mastodon, true);
                        },
                        _ => help::help()
                    }
                }
                if args.len() >= 3 {
                    match args[2].as_ref() {
                        "open" => { 
                            if args.len() == 4 {
                                file::open(args[3].clone());
                                gui(mastodon, true);
                            }
                            else {help::help();}
                        },
                        _ => help::help()
                    }
                }
                //if its only toot, then no image will be taken
                else {gui(mastodon, false);}
            },
            //if first arg is twitter
            "tweet" => {
                if args.len() == 3 {
                    //check any more args
                    match args[2].as_ref() {
                        "area" | "-a" => {
                            file::image(String::from("-s"));
                            gui(twitter, true);
                        },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                            gui(twitter, true);
                        },
                        "full" => {
                            file::image(String::new());
                            gui(twitter, true);
                        },
                        "auth" => println!("NO"),
                        _ => help::help()
                    }
                }
                if args.len() >= 3 {
                    match args[2].as_ref() {
                        "open" => { 
                            if args.len() == 4 {
                                file::open(args[3].clone());
                                gui(twitter, true);
                            }
                            else {help::help();}
                        },
                        _ => help::help()
                    }
                }
                //or just tweet
                else {gui(twitter, false);}
            },
            //if first arg is imgur
            "imgur" => {
                if args.len() == 3 {
                    match args[2].as_ref() {
                        //check for args
                        "area" | "-a" => {
                            file::image(String::from("-s"));
                            imgur::send();
                            },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                            imgur::send();
                        },
                        "full" => {
                            file::image(String::new());
                            imgur::send();
                        },
                        _ => help::help()
                    }
                }
                if args.len() >= 3 {
                    match args[2].as_ref() {
                        "open" => {
                            if args.len() == 4 {
                                file::open(args[3].clone());
                                imgur::send();
                            }
                            else {help::help();}
                        },
                        _ => help::help()
                    }
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
                if args.len() >= 3 {
                    match args[2].as_ref() {
                        "open" => { 
                            if args.len() == 4 {
                                file::open(args[3].clone());
                                match open::that(args[3].clone()) {
                                    Ok(ok) => println!("Opening: {}{:?}", args[3], ok),
                                    Err(e) => panic!("Could not open. {:?}", e),
                                };
                            }
                            else {help::help();}
                        },
                        _ => help::help()
                    }
                }
            },
            //or else give them help, you cant save nothing.
            _ => help::help()
        }
    }
    //if no args at all
    else {help::help();}
}
