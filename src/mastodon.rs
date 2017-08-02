use std::env;
use std::process::*;
use notification;
use Destination;

pub fn image(txt: String)
{
    // gets file to send from temp dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    println!("[Toot]: {}", txt);

    let _ = Command::new("toot")
    .args(&["post", "-m", temp.clone(), &txt])
    .spawn().expect("toot not found");
    notification::image_sent(Destination::new(0), &txt, temp);
}

pub fn toot(txt: String)
{
    println!("[Toot]: {}", txt);
    let _mastodon = Command::new("toot")
    .args(&["post", &txt]).output().expect("toot not found");
    println!("{}", String::from_utf8_lossy(&_mastodon.stdout));
    notification::message_sent(Destination::new(0), &txt);
}