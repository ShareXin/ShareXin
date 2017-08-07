#![allow(unused_variables)]
use std::*;
use std::process::*;
use notification;
use Destination;
use error;

pub fn image(txt: String) {
    // gets file to send from temp dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    println!("[Toot]: {}", txt);

    let _ = match Command::new("toot")
        .args(&["post", "-m", temp.clone(), &txt])
        .spawn()
    {
        Ok(ok) => ok,
        Err(e) => {
            println!("{}", error::message(5));
            process::exit(1)
        }
    };
    notification::image_sent(Destination::new(0), &txt, temp);
}

pub fn toot(txt: String) {
    println!("[Toot]: {}", txt);
    let _mastodon = match Command::new("toot").args(&["post", &txt]).output() {
        Ok(ok) => ok,
        Err(e) => {
            println!("{}", error::message(5));
            process::exit(1)
        }
    };
    println!("{}", String::from_utf8_lossy(&_mastodon.stdout));
    notification::message_sent(Destination::new(0), &txt);
}
