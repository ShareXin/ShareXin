use std::env;
use std::process::*;
use notification;
use Destination;

pub fn image(txt: String)
{
    let mastodon = Destination { twitter: false, mastodon: true, imgur: false };
    //gets file to send from temp dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    let text: &str = &txt.clone()[..];
    println!("[Toot]: {}", txt);
    //uses toot to toot (mastodon api too confusing, but on the TODO)
    let _ = Command::new("toot")
    .args(&["post", "-m", temp.clone(), &txt])
    .spawn().expect("Nope");
    notification::image_sent(mastodon, text, temp);
}

pub fn toot(txt: String)
{
    let mastodon = Destination { twitter: false, mastodon: true, imgur: false };
    let text: &str = &txt.clone()[..];
    println!("[Toot]: {}", txt);
    let _mastodon = Command::new("toot")
    .args(&["post", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_mastodon.stdout));
    notification::message_sent(mastodon, text);
}