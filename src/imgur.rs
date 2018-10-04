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
    let mut file = match File::open(tmp.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(28));
            notification::error(28);
            error::fatal()
        }
    };
    let mut image = Vec::new();
    match file.read_to_end(&mut image) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(28));
            notification::error(28);
            error::fatal()
        }
    };

    // creates imgur app using sharexin app
    let mut copy_link = String::new();
    let handle = Imgur::Handle::new(String::from("37562f83e04fd66"));
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    match handle.upload(&image) {
        Ok(info) => match info.link() {
            Some(link) => copy_link.push_str(link),
            None => {
                eprintln!("{}", error::message(20));
                notification::error(20);
                error::fatal()
            }
        },
        Err(_) => {
            eprintln!("{}", error::message(17));
            notification::error(17);
            error::fatal()
        }
    }
    notification::image_sent(ServiceKind::Imgur, &copy_link, tmp.to_str().unwrap());
    match ctx.set_contents(copy_link.clone()) {
        Ok(ok) => ok,
        Err(_) => eprintln!("{}", error::message(19)),
    };
    match open::that(copy_link) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(19));
            notification::error(19);
            error::fatal()
        }
    };
}
