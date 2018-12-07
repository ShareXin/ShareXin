use notify_rust::{Notification, Timeout};
use std::fs;
use std::{thread, time};
use yaml_rust::YamlLoader;
use crate::{ServiceKind, text};

enum NotificationKind {
    Sent,
    SendFailure,
}

fn notification(service: ServiceKind, notification: NotificationKind) -> String {
    // Gets section of localization file for Notifications
    let mut locator = &YamlLoader::load_from_str(&text::loader()).unwrap()[0]["Notification"];

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
    let notification = match Notification::new()
        .appname("ShareXin")
        .summary(&notification(service, NotificationKind::Sent))
        .body(text)
        .icon(img)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", text::message(23));
            return;
        }
    };

    // Removes temporary file
    if fs::remove_file(img.clone()).is_err() {
        return;
    };

    thread::sleep(time::Duration::from_secs(3));
    notification.close();
}

// Sends a notification when a status is sent
pub fn message_sent(service: ServiceKind, text: &str) {
    let notification = match Notification::new()
        .appname("ShareXin")
        .summary(&notification(service, NotificationKind::Sent))
        .body(text)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", text::message(23));
            return;
        }
    };

    thread::sleep(time::Duration::from_secs(3));
    notification.close();
}

// Sends a notification when a status update didn't go through
pub fn not_sent(service: ServiceKind) {
    if Notification::new()
        .appname("ShareXin")
        .summary(&notification(service, NotificationKind::SendFailure))
        .timeout(Timeout::Milliseconds(3000))
        .show()
        .is_err()
    {
        eprintln!("{}", text::message(23));
        return;
    };
}

// Sends a notification with the error message as the body
pub fn error(code: usize) {
    if Notification::new()
        .appname("ShareXin")
        .summary(&text::message(code))
        .timeout(Timeout::Milliseconds(3000))
        .show()
        .is_err()
    {
        eprintln!("{}", text::message(23));
        return;
    };
}

// Useful when Terminal is not available (if launched without one)
#[allow(dead_code)]
pub fn debug(error: String) {
    if Notification::new()
        .appname("ShareXin")
        .summary(&error.to_string())
        .timeout(Timeout::Milliseconds(3000))
        .show()
        .is_err()
    {
        eprintln!("{}", text::message(23));
        return;
    };
}
