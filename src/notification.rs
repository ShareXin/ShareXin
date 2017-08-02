use libnotify::Notification;
use libnotify;
use Destination;
use language::{language, Language};

// when the tweet/toot with an image is sent
// uses language.rs to get proper language string in your language
pub fn image_sent(service: Destination, text: &str, img: &str)
{
    let string = language(Language::new(service, 0));
    if service.mastodon {
        match libnotify::init("ShareXin") {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to init notification library. {:?}", e),
        };
        let not = Notification::new(string, Some(text), img);
        match not.show() {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to show notification. {:?}", e),
        };
        libnotify::uninit();
    }
    else if service.twitter {
        match libnotify::init("ShareXin") {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to init notification library. {:?}", e),
        };
        let not = Notification::new(string, Some(text), img);
        match not.show() {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to show notification. {:?}", e),
        };
        libnotify::uninit();
    }
    else if service.imgur {
        match libnotify::init("ShareXin") {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to init notification library. {:?}", e),
        };
        let not = Notification::new(string, Some(text), img);
        match not.show() {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to show notification. {:?}", e),
        };
        libnotify::uninit();
    }
}

// when the tweet/toot without an image is sent
// uses language.rs to get proper language string in your language
pub fn message_sent(service: Destination, text: &str)
{
    let string = language(Language::new(service, 0));
    if service.mastodon {
        match libnotify::init("ShareXin") {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to init notification library. {:?}", e),
        };
        let not = Notification::new(string, Some(text), None);
        match not.show() {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to show notification. {:?}", e),
        };
        libnotify::uninit();
    }
    else if service.twitter {
        match libnotify::init("ShareXin") {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to init notification library. {:?}", e),
        };
        let not = Notification::new(string, Some(text), None);
        match not.show() {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to show notification. {:?}", e),
        };
        libnotify::uninit();
    }
}

// when the file has been saved
// uses language.rs to get proper language string in your language
pub fn file_saved()
{
    let string = language(Language::new(Destination::new(3), 2));
    match libnotify::init("ShareXin") {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to init notification library. {:?}", e),
    };
    let not = Notification::new(string, None, None);
    match not.show() {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to show notification. {:?}", e),
    };
    libnotify::uninit();
}

// if a tweet/toot with an image is empty
// uses language.rs to get proper language string in your language
pub fn empty(service: Destination)
{
    let string = language(Language::new(service, 3));
    if service.mastodon {
        match libnotify::init("ShareXin") {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to init notification library. {:?}", e),
        };
        let not = Notification::new(string, None, None);
        match not.show() {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to show notification. {:?}", e),
        };
        libnotify::uninit();
    }
    else if service.twitter {
        match libnotify::init("ShareXin") {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to init notification library. {:?}", e),
        };
        let not = Notification::new(string, None, None);
        match not.show() {
            Ok(ok) => ok,
            Err(e) => panic!("Unable to show notification. {:?}", e),
        };
        libnotify::uninit();
    }
}