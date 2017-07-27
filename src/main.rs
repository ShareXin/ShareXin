extern crate pipers;
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
mod mort;
use gui::gui;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static SHAREXIN: &'static str = "https://crates.io/crates/sharexin";

fn main()
{
    let args: Vec<_> = env::args().collect();
    if args.len() >= 2 {
        match args[1].as_ref() {
            "-V" | "--version" | "version" => println!("sharexin {}", VERSION),
            "-U" | "--upgrade" | "upgrade" => help::upgrade(),
            "toot" => {
                if args.len() == 3 {
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
                else {
                    gui(true, false);
                }
            },
            "tweet" => {
                if args.len() == 3 {
                    println!("Two argument passed");
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
                else {
                    gui(false, false);
                }
            },
            "file" => {
                if args.len() == 3 {
                    println!("Two argument passed");
                    match args[2].as_ref() {
                        "area" | "-a" => {
                            file::image(String::from("-s"));
                        },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                        },
                        "full" => {
                            file::image(String::new());
                        },
                        _ => help::help()
                    }
                }
            },
            _ => help::help()
        }
    }
    else {
        help::help();
    }
}
