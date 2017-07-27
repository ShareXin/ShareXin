extern crate pipers;
extern crate libnotify;
extern crate gtk;
extern crate glib;
extern crate gdk;
use std::*;
mod send;
mod file;
mod help;
mod gui;
use gui::gui;

static VERSION: &'static str = "sharexin 0.3.4";

fn main()
{
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_ref() {
            "-h" | "--help" | "-a" | "-w" | "-n" => help::help(),
            "-V" | "--version" => println!("{}", VERSION),
            "-am" => {
                file::image(String::from("-s"));
                gui(true, true);
            },
            "-at" => {
                file::image(String::from("-s"));
                gui(false, true);
            },
            "-wm" => {
                file::image(String::from("-i"));
                gui(true, true);
            },
            "-wt" => {
                file::image(String::from("-i"));
                gui(false, true);
            },
            "-nt" => gui(false, false),
            "-nm" => gui(true, false),
            "-m" => {
                file::image(String::new());
                gui(true, true);
            },
            "-t" => {
                file::image(String::new());
                gui(false, true);
            },
            "-af" => file::image(String::from("-s")),
            "-wf" => file::image(String::from("-i")),
            "-f" => file::image(String::new()),
            _ => help::help()
        }
    }
    else {
        help::help();
    }
}
