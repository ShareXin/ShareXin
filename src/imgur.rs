use notification;
use Destination;
use std::fs::File;
use std::io::*;
use std::*;
use Imgur;
use open;
use error;

pub fn send() {

    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let mut file = match File::open(tmp.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(28));
            process::exit(1)
        }
    };
    let mut image = Vec::new();
    match file.read_to_end(&mut image) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(28));
            process::exit(1)
        }
    };

    // creates imgur app using sharexin app
    let mut copy_link = String::new();
    let handle = Imgur::Handle::new(String::from("37562f83e04fd66"));

    match handle.upload(&image) {
        Ok(info) => match info.link() {
            Some(link) => copy_link.push_str(link),
            None => {
                eprintln!("{}", error::message(20));
                process::exit(1)
            }
        },
        Err(_) => {
            eprintln!("{}", error::message(17));
            process::exit(1)
        }
    }
    notification::image_sent(Destination::new(2), &copy_link, tmp.to_str().unwrap());
    match open::that(copy_link) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(19));
            process::exit(1)
        }
    };
}
