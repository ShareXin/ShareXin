use std::env;
use std::process::*;
use notification;
use Destination;
use std::io::*;
use std::fs::File;
use auth::gui;
use open;

pub fn image(txt: String)
{
    let twitter = Destination::new(1);
    //gets file to send from temp dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    let text: &str = &txt.clone()[..];
    println!("[Tweet]: {}", txt);
    //t has troubles with empty args, so if txt is empty, it wont be send
    if !txt.is_empty() {
        let _ = Command::new("t")
        .args(&["update", &txt, "-f", temp.clone()]).spawn().expect("Nope");
        notification::image_sent(twitter, text, temp);
    }
    else {
        let _ = Command::new("t")
        .args(&["update", "-f", temp.clone()]).spawn().expect("Nope");
        notification::image_sent(twitter, text, temp);
    }
}

pub fn tweet(txt: String)
{
    let twitter = Destination::new(1);
    let text: &str = &txt.clone()[..];
    println!("[Tweet]: {}", txt);
    let _t = Command::new("t")
    .args(&["update", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_t.stdout));
    notification::message_sent(twitter, text);
}