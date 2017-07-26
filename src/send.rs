extern crate libnotify;
use libnotify::Notification;
use std::env;
use std::process::*;

fn notification_lang<'a>(service: bool, option: u32) -> &'a str
{
    let mut string = "";
    let mut lang = env::var("LANG").unwrap();
    lang = lang.to_lowercase();
    //0 is for notification_0, telling the user their toot/tweet with an image has been sent
    //0 is also for notification_1, telling the user their toot/tweet has been sent
    //2 is for notification_2, telling the user the file is saved
    //3 is for notification_3, telling the user their message-only toot/tweet was empty
    if option == 0 {
        if lang.contains("fr") {
            if service {
                string = "Envoyé vers Mastodon";
            }
            else {
                string = "Envoyé vers Twitter";
            }
        }
        else if lang.contains("de") {
            if service {
                string = "Auf Mastodon veröffentlicht";
            }
            else {
                string = "Auf Twitter veröffentlicht";
            }
        }
        else if lang.contains("es") {
            if service {
                string = "Enviado a Mastodon";
            }
            else {
                string = "Enviado a Twitter";
            }
        }
        else if lang.contains("eo") {
            if service {
                string = "Sendita al Mastodon";
            }
            else {
                string = "Sendita al Twitter";
            }
        }
        else if lang.contains("ja") {
            if service {
                string = "Mastodonに送信";
            }
            else {
                string = "Twitterに送信";
            }
        }
        else {
            if service {
                string = "Sent to Mastodon";
            }
            else {
                string = "Sent to Twitter";
            }
        }
    }
    else if option == 2 {
        if lang.contains("fr") {
            string = "Fichier sauvegardé";
        }
        else if lang.contains("de") {
            string = "Datei gespeichert";
        }
        else if lang.contains("es") {
            string = "Archivo guardado";
        }
        else if lang.contains("eo") {
            string = "Dosieron konservis";
        }
        else if lang.contains("ja") {
            string = "ファイルが保存された";
        }
        else {
            string = "File saved";
        }
    }
    else if option == 3 {
        if lang.contains("fr") {
            if service {
                string = "Toot vide | N'a pas été envoyé";
            }
            else {
                string = "Tweet vide | N'a pas été envoyé";
            }
        }
        else if lang.contains("de") {
            if service {
                string = "Toot leer | Nicht gesendet";
            }
            else {
                string = "Tweet leer | Nicht gesendet";
            }
        }
        else if lang.contains("es") {
            if service {
                string = "Toot vacío | No fue enviado";
            }
            else {
                string = "Tweet vacío | No fue enviado";
            }
        }
        else if lang.contains("eo") {
            if service {
                string = "Malplena Toot | Ne estis sendita";
            }
            else {
                string = "Malplena Tweet | Ne estis sendita";
            }
        }
        else if lang.contains("ja") {
            if service {
                string = "Tootは空です | それはMastodonに送られなかった";
            }
            else {
                string = "Tweetは空です | それはTwitterに送られなかった";
            }
        }
        else {
            if service {
                string = "Toot empty | Not sent";
            }
            else {
                string = "Tweet empty | Not sent";
            }
        }
    }
    string
}

pub fn notification_0(service: bool, text: &str, img: &str)
{
    let string = notification_lang(service.clone(), 0);
    if service {
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new(string, Some(text), img);
        not.show().unwrap();
        libnotify::uninit();
    }
    else {
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new(string, Some(text), img);
        not.show().unwrap();
        libnotify::uninit();
    }
}

pub fn notification_1(service: bool, text: &str)
{
    let string = notification_lang(service.clone(), 0);
    if service {
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new(string, Some(text), None);
        not.show().unwrap();
        libnotify::uninit();
    }
    else {
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new(string, Some(text), None);
        not.show().unwrap();
        libnotify::uninit();
    }
}

pub fn notification_2()
{
    let string = notification_lang(true, 2);
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new(string, None, None);
    not.show().unwrap();
    libnotify::uninit();
}

pub fn notification_3(service: bool)
{
    let string = notification_lang(service.clone(), 3);
    if service {
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new(string, None, None);
        not.show().unwrap();
        libnotify::uninit();
    }
    else {
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new(string, None, None);
        not.show().unwrap();
        libnotify::uninit();
    }
}

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
    notification_0(true, text, temp);
    
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
        notification_0(false, text, temp);
    }
    else {
        let _ = Command::new("t")
        .args(&["update", "-f", temp.clone()]).spawn().expect("Nope");
        notification_0(false, text, temp);
    }
}

pub fn toot(txt: String)
{
    let text: &str = &txt.clone()[..];
    println!("[Toot]: {}", txt);
    let _mastodon = Command::new("toot")
    .args(&["post", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_mastodon.stdout));
    notification_1(true, text);
}

pub fn twitter(txt: String)
{
    let text: &str = &txt.clone()[..];
    println!("[Tweet]: {}", txt);
    let _t = Command::new("t")
    .args(&["update", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_t.stdout));
    notification_1(false, text);
}