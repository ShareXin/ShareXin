use notify_rust::Notification;

use error;
use language::loader;
use std::fs;
use yaml_rust::YamlLoader;
use ServiceKind;

static SOUND: &'static str = "message-new-instant";

#[derive(Debug, Copy, Clone)]
enum NotificationKind {
    Sent,
    File,
    SendFailure,
}

fn notification(service: ServiceKind, notification: NotificationKind) -> String {
    let locators = YamlLoader::load_from_str(loader()).unwrap();
    let mut locator = &locators[0]["Notification"];

    match notification {
        NotificationKind::Sent => {
            locator = &locator["Sent"];
            match service {
                ServiceKind::Twitter => locator = &locator["Twitter"],
                ServiceKind::Mastodon => locator = &locator["Mastodon"],
                ServiceKind::Imgur => locator = &locator["Imgur"],
            }
        }
        NotificationKind::File => locator = &locator["File"],
        NotificationKind::SendFailure => {
            locator = &locator["Not_Sent"];
            match service {
                ServiceKind::Twitter => locator = &locator["Twitter"],
                ServiceKind::Mastodon => locator = &locator["Mastodon"],
                ServiceKind::Imgur => locator = &locator["Imgur"],
            }
        }
    }
    return format!("{}", locator.as_str().unwrap());
}

// when the tweet/toot with an image is sent
pub fn image_sent(service: ServiceKind, text: &str, img: &str) {
    match Notification::new()
        .appname("ShareXin")
        .summary(&notification(service, NotificationKind::Sent))
        .body(text)
        .icon(img)
        .sound_name(SOUND)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            return;
        }
    };

    match fs::remove_file(img.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(0));
            return;
        }
    };
}

// when the tweet/toot without an image is sent
pub fn message_sent(service: ServiceKind, text: &str) {
    match Notification::new()
        .appname("ShareXin")
        .summary(&notification(service, NotificationKind::Sent))
        .body(text)
        .sound_name(SOUND)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            return;
        }
    };
}

// when the file has been saved
pub fn file_saved(img: &str) {
    match Notification::new()
        .appname("ShareXin")
        .summary(&notification(ServiceKind::Twitter, NotificationKind::File))
        .icon(img)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            return;
        }
    };
}

// if a tweet/toot is unable to send
pub fn not_sent(service: ServiceKind) {
    match Notification::new()
        .appname("ShareXin")
        .summary(&notification(service, NotificationKind::SendFailure))
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            return;
        }
    };
}

pub fn error(code: usize) {
    match Notification::new()
        .appname("ShareXin")
        .summary(&error::message(code))
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            return;
        }
    };
}
