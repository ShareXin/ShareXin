#[cfg(target_family = "unix")]
use notify_rust::Notification;

use Destination;
use language::{loader, Language};
use error;
use std::{fs, thread, time};
use yaml_rust::YamlLoader;

fn notification(langue: Language) -> String {

    let file = loader();
    let locators = YamlLoader::load_from_str(file).unwrap();
    let mut locator = &locators[0]["Notification"];

    if langue.option == 0 {
        locator = &locator["Sent"];
        if langue.service.mastodon {
            locator = &locator["Mastodon"];
        } else if langue.service.twitter {
            locator = &locator["Twitter"];
        } else if langue.service.imgur {
            locator = &locator["Imgur"];
        }
    } else if langue.option == 2 {
        locator = &locator["File"];
    } else if langue.option == 3 {
        locator = &locator["Empty"];
        if langue.service.mastodon {
            locator = &locator["Mastodon"];
        } else if langue.service.twitter {
            locator = &locator["Twitter"];
        }
    } else if langue.option == 4 {
        locator = &locator["Not_Sent"];
        if langue.service.mastodon {
            locator = &locator["Mastodon"];
        } else if langue.service.twitter {
            locator = &locator["Twitter"];
        }
    }
    return format!("{}", locator.as_str().unwrap());
}


// when the tweet/toot with an image is sent
pub fn image_sent(service: Destination, text: &str, img: &str) {

    let string = notification(Language::new(service, 0));
    match Notification::new()
        .appname("ShareXin")
        .summary(&string)
        .body(text)
        .icon(img)
        .timeout(5000)
        .show() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            error(23);
            return;
        }
    };

    thread::sleep(time::Duration::new(5, 0));

    match fs::remove_file(img.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(0));
            error(0);
            return;
        }
    };

}

// when the tweet/toot without an image is sent
pub fn message_sent(service: Destination, text: &str) {

    let string = notification(Language::new(service, 0));
    match Notification::new()
        .appname("ShareXin")
        .summary(&string)
        .body(text)
        .timeout(5000)
        .show() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            error(23);
            return;
        }
    };

}

// when the file has been saved
pub fn file_saved(img: &str) {

    let string = notification(Language::new(Destination::new(3), 2));
    match Notification::new()
        .appname("ShareXin")
        .summary(&string)
        .icon(img)
        .timeout(5000)
        .show() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            error(23);
            return;
        }
    };

}

// if a tweet/toot without an image is empty
pub fn empty(service: Destination) {

    let string = notification(Language::new(service, 3));
    match Notification::new()
        .appname("ShareXin")
        .summary(&string)
        .timeout(5000)
        .show() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            error(23);
            return;
        }
    };

}

// if a tweet/toot is unable to send
pub fn not_sent(service: Destination) {

    let string = notification(Language::new(service, 4));
    match Notification::new()
        .appname("ShareXin")
        .summary(&string)
        .timeout(5000)
        .show() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            error(23);
            return;
        }
    };

}

pub fn error(code: usize) {

    let message = error::message(code);
    match Notification::new()
        .appname("ShareXin")
        .summary(&message)
        .timeout(5000)
        .show() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            return;
        }
    };

}
