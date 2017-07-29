extern crate libnotify;
use libnotify::Notification;
use Destination;
use language::{language, Language};

pub fn image_sent(service: Destination, text: &str, img: &str)
{
    let langue = Language {
        service: service,
        option: 0,
    };
    //when the tweet/toot with an image is sent
    let string = language(langue);
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

pub fn message_sent(service: Destination, text: &str)
{
    let langue = Language {
        service: service,
        option: 0,
    };
    //when the tweet/toot without an image is sent
    let string = language(langue);
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

pub fn file_saved()
{
    let langue = Language {
        service: Destination {
        twitter: false, mastodon: false, 
        imgur: false},
        option: 2,
    };
    //when the file has been saved
    let string = language(langue);
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

pub fn empty(service: Destination)
{
    let langue = Language {
        service: service,
        option: 3,
    };
    //if tweet/toot is empty
    let string = language(langue);
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