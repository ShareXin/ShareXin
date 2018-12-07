use open;
use std::fs::File;
use std::io::Read;
use imgur;
use crate::{image, notification, text, ServiceKind};

pub fn send() {
    let tmp = image::temp_dir();

    // Opens file for use
    let mut file = match File::open(tmp.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", text::message(28));
            notification::error(28);
            text::exit()
        }
    };

    // Stores image in a Vector
    let mut image = Vec::new();
    if file.read_to_end(&mut image).is_err() {
        eprintln!("{}", text::message(28));
        notification::error(28);
        text::exit();
    };

    // Creates Imgur Applications for sending to Imgur API
    let mut copy_link = String::new();
    let handle = imgur::Handle::new(String::from("37562f83e04fd66"));

    // Uploads file to Imgur API
    match handle.upload(&image) {
        Ok(info) => match info.link() {
            Some(link) => copy_link.push_str(link),
            None => {
                eprintln!("{}", text::message(20));
                notification::error(20);
                text::exit()
            }
        },
        Err(_) => {
            eprintln!("{}", text::message(17));
            notification::error(17);
            text::exit()
        }
    }

    // Send notification
    notification::image_sent(ServiceKind::Imgur, &copy_link, tmp.to_str().unwrap());

    // Opens url
    if open::that(copy_link).is_err() {
        eprintln!("{}", text::message(19));
        notification::error(19);
        text::exit();
    };
}
