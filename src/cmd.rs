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

pub fn cmd() {

    let twitter = Destination::new(1);
    let mastodon = Destination::new(0);

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].as_ref() {
                "-V" | "--version" | "version" => println!("sharexin {}", VERSION),
                "-U" | "--upgrade" | "upgrade" => upgrade::upgrade(),
                "toot" => dialog(mastodon, false),
                "tweet" => dialog(twitter, false),
                _ => language::help(),
            }
        }
        3 => {
            match args[1].as_ref() {
                "toot" => {
                    match args[2].as_ref() {
                        "area" => {
                            image::image(0);
                            dialog(mastodon, true);
                        }
                        "window" => {
                            image::image(1);
                            dialog(mastodon, true);
                        }
                        "full" => {
                            image::image(2);
                            dialog(mastodon, true);
                        }
                        "auth" => mastodon::auth(),
                        _ => language::help(),
                    }
                }
                "tweet" => {
                    match args[2].as_ref() {
                        "area" => {
                            image::image(0);
                            dialog(twitter, true);
                        }
                        "window" => {
                            image::image(1);
                            dialog(twitter, true);
                        }
                        "full" => {
                            image::image(2);
                            dialog(twitter, true);
                        }
                        "auth" => twitter::auth(),
                        _ => language::help(),
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
                        "file" => {
                            save::file(args[3].clone());
                            dialog(mastodon, true);
                        }
                        _ => language::help(),
                    }
                }
                "tweet" => {
                    match args[2].as_ref() {
                        "file" => {
                            save::file(args[3].clone());
                            dialog(twitter, true);
                        }
                        _ => language::help(),
                    }
                }
                "imgur" => {
                    match args[2].as_ref() {
                        "file" => {
                            save::file(args[3].clone());
                            imgur::send();
                        }
                        _ => language::help(),
                    }
                }
                _ => language::help(),
            }
        }
        _ => language::help(),
    }
}
