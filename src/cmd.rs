use Destination;
use std::{env, fs};
use gui::gui;
use image;
use imgur;
use VERSION;
use language;
use error;
use upgrade;
use twitter;
use mastodon;
use save;

pub fn cmd() {

    let twitter = Destination::new(1);
    let mastodon = Destination::new(0);

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].as_ref() {
                "-V" | "--version" | "version" => println!("sharexin {}", VERSION),
                "-U" | "--upgrade" | "upgrade" => upgrade::upgrade(),
                "toot" => gui(mastodon, false),
                "tweet" => gui(twitter, false),
                _ => check(),
            }
        }
        3 => {
            match args[1].as_ref() {
                "toot" => {
                    match args[2].as_ref() {
                        "area" => {
                            image::image(0);
                            gui(mastodon, true);
                        }
                        "window" => {
                            image::image(1);
                            gui(mastodon, true);
                        }
                        "full" => {
                            image::image(2);
                            gui(mastodon, true);
                        }
                        "auth" => mastodon::auth(),
                        _ => check(),
                    }
                }
                "tweet" => {
                    match args[2].as_ref() {
                        "area" => {
                            image::image(0);
                            gui(twitter, true);
                        }
                        "window" => {
                            image::image(1);
                            gui(twitter, true);
                        }
                        "full" => {
                            image::image(2);
                            gui(twitter, true);
                        }
                        "auth" => twitter::auth(),
                        _ => check(),
                    }
                }
                "imgur" => {
                    match args[2].as_ref() {
                        "area" => {
                            image::image(0);
                            imgur::send();
                        }
                        "window" => {
                            image::image(1);
                            imgur::send();
                        }
                        "full" => {
                            image::image(2);
                            imgur::send();
                        }
                        _ => check(),
                    }
                }
                _ => check(),
            }
        }
        4 => {
            match args[1].as_ref() {
                "toot" => {
                    match args[2].as_ref() {
                        "file" => {
                            save::file(args[3].clone());
                            gui(mastodon, true);
                        }
                        _ => check(),
                    }
                }
                "tweet" => {
                    match args[2].as_ref() {
                        "file" => {
                            save::file(args[3].clone());
                            gui(twitter, true);
                        }
                        _ => check(),
                    }
                }
                "imgur" => {
                    match args[2].as_ref() {
                        "file" => {
                            save::file(args[3].clone());
                            imgur::send();
                        }
                        _ => check(),
                    }
                }
                _ => check(),
            }
        }
        _ => check(),
    }
}

fn check() {

    println!("{}", language::help());
    if !check_exists("t") {
        eprintln!("\n{}", error::message(5));
    }
    if !check_exists("toot") {
        eprintln!("{}", error::message(6));
    }
    if !check_exists("convert") && !cfg!(target_os = "macos") {
        eprintln!("{}", error::message(15));
    }

}

fn check_exists(program: &str) -> bool {

    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    return false;

}
