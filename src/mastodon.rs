use error;
use image;
use notification;
use std::process::Command;
use ServiceKind;

pub fn image(status: String) {
    let service = ServiceKind::Mastodon;

    let tmp = image::temp_dir();
    let temp = tmp.to_str().unwrap().clone();

    let _toot = match Command::new("toot")
        .args(&["post", "-m", &temp, &status])
        .status()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            notification::not_sent(service);
            error::exit()
        }
    };
    if _toot.code() == Some(2) {
        eprintln!("{}", error::message(21));
        notification::not_sent(service);
        error::exit();
    } else {
        notification::image_sent(service, &status, temp);
    }
}

pub fn toot(status: String) {
    let service = ServiceKind::Mastodon;

    let _toot = match Command::new("toot").args(&["post", &status]).status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            notification::not_sent(service);
            error::exit()
        }
    };
    if _toot.code() == Some(2) {
        eprintln!("{}", error::message(21));
        notification::not_sent(service);
        error::exit();
    } else {
        notification::message_sent(service, &status);
    }
}

pub fn auth() {
    match Command::new("toot").arg("login_browser").status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            error::exit()
        }
    };
}
