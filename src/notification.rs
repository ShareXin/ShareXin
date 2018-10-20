use notify_rust::{Notification, Timeout};

use error;
use language::loader;
use std::fs;
use yaml_rust::YamlLoader;
use ServiceKind;

enum NotificationKind {
    Sent,
    SendFailure,
}

fn notification(service: ServiceKind, notification: NotificationKind) -> String {
    // Gets section of localization file for Notifications
    let mut locator = &YamlLoader::load_from_str(&loader()).unwrap()[0]["Notification"];

    // Checks kind of notification and the kind of service (Twitter, Mastodon, Imgur) used
    match notification {
        NotificationKind::Sent => {
            locator = &locator["Sent"];
            match service {
                ServiceKind::Twitter => locator = &locator["Twitter"],
                ServiceKind::Mastodon => locator = &locator["Mastodon"],
                ServiceKind::Imgur => locator = &locator["Imgur"],
            }
        }
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

// Sends a notification with notify-rust, when a status with an image or an image is sent/uploaded
pub fn image_sent(service: ServiceKind, text: &str, img: &str) {
    match Notification::new()
        .appname("ShareXin")
        .summary(&notification(service, NotificationKind::Sent))
        .body(text)
        .icon(img)
        .timeout(Timeout::Milliseconds(2000))
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            return;
        }
    };

    // Removes temporary file
    match fs::remove_file(img.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(0));
            return;
        }
    };
}

// Sends a notification when a status is sent
pub fn message_sent(service: ServiceKind, text: &str) {
    match Notification::new()
        .appname("ShareXin")
        .summary(&notification(service, NotificationKind::Sent))
        .body(text)
        .timeout(Timeout::Milliseconds(2000))
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(23));
            return;
        }
    };
}

// Sends a notification when a status update didn't go through
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

// Sends a notification with the error message as the body
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
