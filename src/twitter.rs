use std::process::Command;
use crate::{image, notification, text, ServiceKind};

pub fn image(status: String) {
    let service = ServiceKind::Twitter;

    let tmp = image::temp_dir();
    let temp = tmp.to_str().unwrap().clone();

    // Calls the "t" Ruby app and sends a staus with an image
    let t = match Command::new("t")
        .args(&["update", &status, "-f", &temp])
        .status()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", text::message(5));
            notification::not_sent(service);
            text::exit()
        }
    };

    // If t gives the error code 1, then the status was not sent
    if t.code() == Some(1) {
        eprintln!("{}", text::message(22));
        notification::not_sent(service);
        text::exit();
    } else {
        notification::image_sent(service, &status, temp);
    }
}

pub fn tweet(status: String) {
    let service = ServiceKind::Twitter;

    // Calls the "t" Ruby app and sends a staus
    let t = match Command::new("t").args(&["update", &status]).status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", text::message(5));
            notification::not_sent(service);
            text::exit()
        }
    };

    // If t gives the error code 1, then the status was not sent
    if t.code() == Some(1) {
        eprintln!("{}", text::message(22));
        notification::not_sent(service);
        text::exit();
    } else {
        notification::message_sent(service, &status);
    }
}

pub fn auth() {
    // Calls the "t" Ruby app and asks the user to login
    if Command::new("t").arg("authorize").status().is_err() {
        eprintln!("{}", text::message(5));
        text::exit();
    };
}
