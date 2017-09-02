use std::process::Command;
use notification;
use Destination;
use error;
use image;

pub fn image(txt: String) {

    let twitter = Destination::new(1);

    let tmp = image::temp_dir(0);
    let temp = tmp.to_str().unwrap().clone();

    let _t = match Command::new("t")
        .args(&["update", &txt, "-f", &temp])
        .status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(5));
            notification::not_sent(twitter);
            error::fatal()
        }
    };

    if _t.code() == Some(1) {
        eprintln!("{}", error::message(22));
        notification::not_sent(twitter);
        error::fatal();
    }
    notification::image_sent(twitter, &txt, temp);
}

pub fn tweet(txt: String) {

    let twitter = Destination::new(1);

    let _t = match Command::new("t").args(&["update", &txt]).status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(5));
            notification::not_sent(twitter);
            error::fatal()
        }
    };

    if _t.code() == Some(1) {
        eprintln!("{}", error::message(22));
        notification::not_sent(twitter);
        error::fatal();
    }

    notification::message_sent(twitter, &txt);
}

pub fn auth() {

    match Command::new("t").arg("authorize").status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(5));
            error::fatal()
        }
    };

}
