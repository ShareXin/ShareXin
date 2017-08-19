use std::process::Command;
use notification;
use Destination;
use error;
use image;

pub fn image(txt: String) {

    let mastodon = Destination::new(0);

    let tmp = image::temp_dir(0);
    let temp = tmp.to_str().unwrap().clone();

    let _toot = match Command::new("toot")
        .args(&["post", "-m", &temp, &txt])
        .status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            notification::not_sent(mastodon);
            error::fatal()
        }
    };
    if _toot.code() == Some(2) {
        eprintln!("{}", error::message(21));
        notification::not_sent(mastodon);
        error::fatal();
    }
    notification::image_sent(mastodon, &txt, temp);
}

pub fn toot(txt: String) {

    let mastodon = Destination::new(0);

    let _toot = match Command::new("toot").args(&["post", &txt]).status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            notification::not_sent(mastodon);
            error::fatal()
        }
    };
    if _toot.code() == Some(2) {
        eprintln!("{}", error::message(21));
        notification::not_sent(mastodon);
        error::fatal();
    }
    notification::message_sent(mastodon, &txt);
}

pub fn auth() {

    match Command::new("toot").arg("login").status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            error::fatal()
        }
    };

}
