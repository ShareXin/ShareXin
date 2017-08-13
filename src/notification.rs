#[cfg(target_family = "unix")]
use notify_rust::Notification;

use Destination;
use language;
use language::*;

// when the tweet/toot with an image is sent
pub fn image_sent(service: Destination, text: &str, img: &str) {
    let string = notification(Language::new(service, 0));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .body(text)
        .icon(img)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(23));
            return;
        }
    };
}

// when the tweet/toot without an image is sent
pub fn message_sent(service: Destination, text: &str) {
    let string = notification(Language::new(service, 0));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .body(text)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(23));
            return;
        }
    };
}

// when the file has been saved
pub fn file_saved() {
    let string = notification(Language::new(Destination::new(3), 2));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(23));
            return;
        }
    };
}

// if a tweet/toot without an image is empty
pub fn empty(service: Destination) {
    let string = notification(Language::new(service, 3));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(23));
            return;
        }
    };
}

// if a tweet/toot is unable to send
pub fn not_sent(service: Destination) {
    let string = notification(Language::new(service, 4));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(23));
            return;
        }
    };
}
