use Destination;
use std::*;
use gui::gui;
use help;
use image;
use imgur;
use VERSION;
use upgrade;

pub fn cmd() {
    let twitter = Destination::new(1);
    let mastodon = Destination::new(0);

    // fetches user arguments
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => match args[1].as_ref() {
            "-V" | "--version" | "version" => println!("sharexin {}", VERSION),
            "-U" | "--upgrade" | "upgrade" => upgrade::upgrade(),
            "toot" => gui(mastodon, false),
            "tweet" => gui(twitter, false),
            _ => help::help(),
        },
        3 => match args[1].as_ref() {
            "toot" => match args[2].as_ref() {
                "area" => {
                    image::image(String::from("-s"));
                    gui(mastodon, true);
                }
                "window" => {
                    image::image(String::from("-i"));
                    gui(mastodon, true);
                }
                "full" => {
                    image::image(String::new());
                    gui(mastodon, true);
                }
                _ => help::help(),
            },
            "tweet" => match args[2].as_ref() {
                "area" => {
                    image::image(String::from("-s"));
                    gui(twitter, true);
                }
                "window" => {
                    image::image(String::from("-i"));
                    gui(twitter, true);
                }
                "full" => {
                    image::image(String::new());
                    gui(twitter, true);
                }
                _ => help::help(),
            },
            "imgur" => match args[2].as_ref() {
                "area" => {
                    image::image(String::from("-s"));
                    imgur::send();
                }
                "window" => {
                    image::image(String::from("-i"));
                    imgur::send();
                }
                "full" => {
                    image::image(String::new());
                    imgur::send();
                }
                _ => help::help(),
            },
            _ => help::help(),
        },
        4 => match args[1].as_ref() {
            "toot" => match args[2].as_ref() {
                "file" => {
                    image::file(args[3].clone());
                    gui(mastodon, true);
                }
                _ => help::help(),
            },
            "tweet" => match args[2].as_ref() {
                "file" => {
                    image::file(args[3].clone());
                    gui(twitter, true);
                }
                _ => help::help(),
            },
            "imgur" => match args[2].as_ref() {
                "file" => {
                    image::file(args[3].clone());
                    imgur::send();
                }
                _ => help::help(),
            },
            _ => help::help(),
        },
        _ => help::help(),
    }
}
