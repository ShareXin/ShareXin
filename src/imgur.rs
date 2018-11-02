use clipboard::{ClipboardContext, ClipboardProvider};
use error;
use image;
use notification;
use open;
use std::fs::File;
use std::io::Read;
use Imgur;
use ServiceKind;

pub fn send() {
    let tmp = image::temp_dir();

    // Opens file for use
    let mut file = match File::open(tmp.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(28));
            notification::error(28);
            error::exit()
        }
    };

    // Stores image in a Vector
    let mut image = Vec::new();
    if file.read_to_end(&mut image).is_err() {
        eprintln!("{}", error::message(28));
        notification::error(28);
        error::exit();
    };

    // Creates Imgur Applications for sending to Imgur API
    let mut copy_link = String::new();
    let handle = Imgur::Handle::new(String::from("37562f83e04fd66"));
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    // Uploads file to Imgur API
    match handle.upload(&image) {
        Ok(info) => match info.link() {
            Some(link) => copy_link.push_str(link),
            None => {
                eprintln!("{}", error::message(20));
                notification::error(20);
                error::exit()
            }
        },
        Err(_) => {
            eprintln!("{}", error::message(17));
            notification::error(17);
            error::exit()
        }
    }

    // Send notification
    notification::image_sent(ServiceKind::Imgur, &copy_link, tmp.to_str().unwrap());

    // Copies url to clipboard
    if ctx.set_contents(copy_link.clone()).is_err() {
        eprintln!("{}", error::message(19));
    };

    // Opens url (sort of a workaround for Wayland clipboard
    // deleting contents of clipboard after an app closes)
    if open::that(copy_link).is_err() {
        eprintln!("{}", error::message(19));
        notification::error(19);
        error::exit();
    };
}
