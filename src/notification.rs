#[cfg(target_os = "linux")]
use notify_rust::Notification;

#[cfg(target_os = "macos")]
use mac_notification_sys::*;

use Destination;
use language::{language, Language};

// when the tweet/toot with an image is sent
// uses language.rs to get proper language string in your language
#[cfg(target_os = "linux")]
pub fn image_sent(service: Destination, text: &str, img: &str) {
    let string = language(Language::new(service, 0));
    Notification::new()
        .appname("ShareXin")
        .summary(string)
        .body(text)
        .icon(img)
        .timeout(5000)
        .show()
        .unwrap();
}

// when the tweet/toot without an image is sent
// uses language.rs to get proper language string in your language
#[cfg(target_os = "linux")]
pub fn message_sent(service: Destination, text: &str) {
    let string = language(Language::new(service, 0));
    Notification::new()
        .appname("ShareXin")
        .summary(string)
        .body(text)
        .timeout(5000)
        .show()
        .unwrap();
}

// when the file has been saved
// uses language.rs to get proper language string in your language
#[cfg(target_os = "linux")]
pub fn file_saved() {
    let string = language(Language::new(Destination::new(3), 2));
    Notification::new()
        .appname("ShareXin")
        .summary(string)
        .timeout(5000)
        .show()
        .unwrap();
}

// if a tweet/toot with an image is empty
// uses language.rs to get proper language string in your language
#[cfg(target_os = "linux")]
pub fn empty(service: Destination) {
    let string = language(Language::new(service, 3));
    Notification::new()
        .appname("ShareXin")
        .summary(string)
        .timeout(5000)
        .show()
        .unwrap();
}

#[cfg(target_os = "macos")]
pub fn image_sent(service: Destination, text: &str, img: &str) {
    let string = language(Language::new(service, 0));
    let bundle = mac_notification_sys::get_bundle_identifier("ShareXin").unwrap();
    set_application(&bundle).unwrap();
    send_notification("ShareXin", &Some(&service.name()), string, &None).unwrap();
}

#[cfg(target_os = "macos")]
pub fn message_sent(service: Destination, text: &str) {
    let string = language(Language::new(service, 0));
    let bundle = mac_notification_sys::get_bundle_identifier("ShareXin").unwrap();
    set_application(&bundle).unwrap();
    send_notification("ShareXin", &Some(&service.name()), string, &None).unwrap();
}

#[cfg(target_os = "macos")]
pub fn file_saved() {
    let string = language(Language::new(Destination::new(3), 2));
    let bundle = mac_notification_sys::get_bundle_identifier("ShareXin").unwrap();
    set_application(&bundle).unwrap();
    send_notification("ShareXin", &None, string, &None).unwrap();
}

#[cfg(target_os = "macos")]
pub fn empty(service: Destination) {
    let string = language(Language::new(service, 3));
    let bundle = mac_notification_sys::get_bundle_identifier("ShareXin").unwrap();
    set_application(&bundle).unwrap();
    send_notification("ShareXin", &Some(&service.name()), string, &None).unwrap();
}
