use Destination;
use std::env;
use dialog::dialog;
use image;
use imgur;
use VERSION;
use language;
use upgrade;
use twitter;
use mastodon;
use save;
use tray;
use sharexin::*;

pub fn cmd() {

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].as_ref() {
                "-V" | "--version" | "version" => println!("sharexin {}", VERSION),
                "-U" | "--upgrade" | "upgrade" => upgrade::upgrade(),
                "toot" => toot(),
                "tweet" => tweet(),
                "-t" | "--tray" => tray::tray(),
                _ => language::help(),
            }
        }
        3 => {
            match args[1].as_ref() {
                "toot" => {
                    match args[2].as_ref() {
                        "area" => toot_area(),
                        "window" => toot_window(),
                        "full" => toot_full(),
                        "auth" => mastodon::auth(),
                        _ => language::help(),
                    }
                }
                "tweet" => {
                    match args[2].as_ref() {
                        "area" => tweet_area(),
                        "window" => tweet_window(),
                        "full" => tweet_full(),
                        "auth" => twitter::auth(),
                        _ => language::help(),
                    }
                }
                "imgur" => {
                    match args[2].as_ref() {
                        "area" => imgur_area(),
                        "window" => imgur_window(),
                        "full" => imgur_full(),
                        _ => language::help(),
                    }
                }
                _ => language::help(),
            }
        }
        4 => {
            match args[1].as_ref() {
                "toot" => {
                    match args[2].as_ref() {
                        "file" => toot_file(args[3].clone()),
                        _ => language::help(),
                    }
                }
                "tweet" => {
                    match args[2].as_ref() {
                        "file" => tweet_file(args[3].clone()),
                        _ => language::help(),
                    }
                }
                "imgur" => {
                    match args[2].as_ref() {
                        "file" => imgur_file(args[3].clone()),
                        _ => language::help(),
                    }
                }
                _ => language::help(),
            }
        }
        _ => language::help(),
    }
}
