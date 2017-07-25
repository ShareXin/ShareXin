extern crate libnotify;
use libnotify::Notification;
use std::env;
use std::process::*;

pub fn toot_img(txt: String)
{
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    let text: &str = &txt.clone()[..];
    println!("[Toot]: {}", txt);
    let _ = Command::new("toot")
    .args(&["post", "-m", temp.clone(), &txt])
    .spawn().expect("Nope");
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new("Sent to Mastodon", Some(text), temp);
    not.show().unwrap();
    libnotify::uninit();
    println!("[Notification]");
}

pub fn twitter_img(txt: String)
{
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    let text: &str = &txt.clone()[..];
    println!("[Tweet]: {}", txt);
    if !txt.is_empty() {
        let _ = Command::new("t")
        .args(&["update", &txt, "-f", temp.clone()]).spawn().expect("Nope");
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new("Sent to Twitter", Some(text), temp);
        not.show().unwrap();
        libnotify::uninit();
        println!("[Notification]");
    }
    else {
        let _ = Command::new("t")
        .args(&["update", "-f", temp.clone()]).spawn().expect("Nope");
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new("Sent to Twitter", Some(text), temp);
        not.show().unwrap();
        libnotify::uninit();
        println!("[Notification]");
    }
}

pub fn toot(txt: String)
{
    let text: &str = &txt.clone()[..];
    println!("[Toot]: {}", txt);
    let _mastodon = Command::new("toot")
    .args(&["post", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_mastodon.stdout));
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new("Sent to Mastodon", Some(text), None);
    not.show().unwrap();
    libnotify::uninit();
    println!("[Notification]");
}

pub fn twitter(txt: String)
{
    let text: &str = &txt.clone()[..];
    println!("[Tweet]: {}", txt);
    let _t = Command::new("t")
    .args(&["update", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_t.stdout));
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new("Sent to Twitter", Some(text), None);
    not.show().unwrap();
    libnotify::uninit();
    println!("[Notification]");
}