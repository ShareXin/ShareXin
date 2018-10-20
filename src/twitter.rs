use error;
use image;
use notification;
use std::process::Command;
use ServiceKind;

pub fn image(status: String) {
    let service = ServiceKind::Twitter;

    let tmp = image::temp_dir();
    let temp = tmp.to_str().unwrap().clone();

    // Calls the "t" Ruby app and sends a staus with an image
    let _t = match Command::new("t")
        .args(&["update", &status, "-f", &temp])
        .status()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(5));
            notification::not_sent(service);
            error::exit()
        }
    };

    // If t gives the error code 1, then the status was not sent
    if _t.code() == Some(1) {
        eprintln!("{}", error::message(22));
        notification::not_sent(service);
        error::exit();
    } else {
        notification::image_sent(service, &status, temp);
    }
}

pub fn tweet(status: String) {
    let service = ServiceKind::Twitter;

    // Calls the "t" Ruby app and sends a staus
    let _t = match Command::new("t").args(&["update", &status]).status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(5));
            notification::not_sent(service);
            error::exit()
        }
    };

    // If t gives the error code 1, then the status was not sent
    if _t.code() == Some(1) {
        eprintln!("{}", error::message(22));
        notification::not_sent(service);
        error::exit();
    } else {
        notification::message_sent(service, &status);
    }
}

pub fn auth() {
    // Calls the "t" Ruby app and asks the user to login
    match Command::new("t").arg("authorize").status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(5));
            error::exit()
        }
    };
}
