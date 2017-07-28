use std::env;
use std::process::*;
use notification;

pub fn toot_img(txt: String)
{
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
    notification::image_sent(true, text, temp);
}

pub fn twitter_img(txt: String)
{
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
        notification::image_sent(false, text, temp);
    }
    else {
        let _ = Command::new("t")
        .args(&["update", "-f", temp.clone()]).spawn().expect("Nope");
        notification::image_sent(false, text, temp);
    }
}

pub fn toot(txt: String)
{
    let text: &str = &txt.clone()[..];
    println!("[Toot]: {}", txt);
    let _mastodon = Command::new("toot")
    .args(&["post", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_mastodon.stdout));
    notification::message_sent(true, text);
}

pub fn twitter(txt: String)
{
    let text: &str = &txt.clone()[..];
    println!("[Tweet]: {}", txt);
    let _t = Command::new("t")
    .args(&["update", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_t.stdout));
    notification::message_sent(false, text);
}
