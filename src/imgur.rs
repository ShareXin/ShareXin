use notification;
use Destination;
use std::fs::File;
use std::io::*;
use std::*;
use Imgur;
use open;

pub fn send() {
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let mut file = File::open(tmp.clone()).expect("Could not open image.");
    let mut image = Vec::new();
    file.read_to_end(&mut image).expect("Could not read image file.");
    let id = String::from("37562f83e04fd66");
    let mut copy_link = String::new();
    let handle = Imgur::Handle::new(id);

    match handle.upload(&image) {
        Ok(info) => {
            match info.link() {
                Some(link) => copy_link.push_str(link),
                None => println!("Unknown error."),
            }
        }
        Err(e) => println!("Error uploading: {:?}", e),
    }
    let imgur = Destination::new(2);
    notification::image_sent(imgur, &copy_link, tmp.to_str().unwrap());
    match open::that(copy_link) {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to open imgur. {:?}", e),
    };
}