#[cfg(target_family = "unix")]
use notify_rust::Notification;

use Destination;
use error;
use language::{language, Language};

// when the tweet/toot with an image is sent
// uses language.rs to get proper language string in your language
pub fn image_sent(service: Destination, text: &str, img: &str) {
    let string = language(Language::new(service, 0));
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
            eprintln!("Error 28: {}", error::message(28));
            return;
        }
    };
}

// when the tweet/toot without an image is sent
// uses language.rs to get proper language string in your language
pub fn message_sent(service: Destination, text: &str) {
    let string = language(Language::new(service, 0));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .body(text)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("Error 28: {}", error::message(28));
            return;
        }
    };
}

// when the file has been saved
// uses language.rs to get proper language string in your language
pub fn file_saved() {
    let string = language(Language::new(Destination::new(3), 2));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("Error 28: {}", error::message(28));
            return;
        }
    };
}

// if a tweet/toot with an image is empty
// uses language.rs to get proper language string in your language
pub fn empty(service: Destination) {
    let string = language(Language::new(service, 3));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("Error 28: {}", error::message(28));
            return;
        }
    };
}

// if a tweet/toot is unable to send
// uses language.rs to get proper language string in your language
pub fn not_sent(service: Destination) {
    let string = language(Language::new(service, 4));
    match Notification::new()
        .appname("ShareXin")
        .summary(string)
        .timeout(5000)
        .show()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("Error 28: {}", error::message(28));
            return;
        }
    };
}
