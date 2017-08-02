use notification;
use Destination;
use std::fs::File;
use std::io::*;
use std::*;
use Imgur;
use open;

pub fn send() {

    // gets the temporary file
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let mut file = File::open(tmp.clone()).expect("Could not open image.");
    let mut image = Vec::new();
    file.read_to_end(&mut image).expect("Could not read image file.");

    // sharexin's imgur app, don't abuse it!!!
    let mut copy_link = String::new();
    let handle = Imgur::Handle::new(String::from("37562f83e04fd66"));

    match handle.upload(&image) {
        Ok(info) => {
            match info.link() {
                Some(link) => copy_link.push_str(link),
                None => println!("Unknown error."),
            }
        }
        Err(e) => println!("Error uploading: {:?}", e),
    }
    notification::image_sent(Destination::new(2), &copy_link, tmp.to_str().unwrap());
    match open::that(copy_link) {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to open imgur. {:?}", e),
    };
}