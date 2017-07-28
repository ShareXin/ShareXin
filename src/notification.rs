extern crate libnotify;
use libnotify::Notification;
use std::env;

fn language<'a>(service: bool, option: u32) -> &'a str
{
    //string creates result string, modified later
    let mut string = "";
    //gets user var for language (hopefully?)
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
        else if lang.contains("cn") {
            if service {
                string = "它已被发送到Mastodon";
            }
            else {
                string = "它已被发送到Twitter";
            }
        }
        else if lang.contains("tw") {
            if service {
                string = "它已被發送到Mastodon";
            }
            else {
                string = "它已被發送到Twitter";
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
        else if lang.contains("ko") {
            if service {
                string = "Mastodon에 보냈습니다";
            }
            else {
                string = "Twitter에 보냈습니다";
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
        else if lang.contains("cn") {
            string = "文件已保存";
        }
        else if lang.contains("tw") {
            string = "文件已保存";
        }
        else if lang.contains("ja") {
            string = "ファイルが保存された";
        }
        else if lang.contains("ko") {
            string = "파일이 저장된";
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
        else if lang.contains("cn") {
            if service {
                string = "Toot是空 | 它不会发送到Mastodon";
            }
            else {
                string = "Tweet是空 | 它不会发送到Twitter";
            }
        }
        else if lang.contains("tw") {
            if service {
                string = "Toot是空 | 它不會發送到Mastodon";
            }
            else {
                string = "Tweet是空 | 它不會發送到Twitter";
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
        else if lang.contains("ko") {
            if service {
                string = "Toot 비어 있습니다 | 그것은 Mastodon에 보내지지 않았다";
            }
            else {
                string = "Tweet 비어 있습니다 | 그것은 Twitter에 보내지지 않았다";
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

pub fn image_sent(service: bool, text: &str, img: &str)
{
    //when the tweet/toot with an image is sent
    let string = language(service.clone(), 0);
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

pub fn message_sent(service: bool, text: &str)
{
    //when the tweet/toot without an image is sent
    let string = language(service.clone(), 0);
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

pub fn file_saved()
{
    //when the file has been saved
    let string = language(true, 2);
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new(string, None, None);
    not.show().unwrap();
    libnotify::uninit();
}

pub fn empty(service: bool)
{
    //if tweet/toot is empty
    let string = language(service.clone(), 3);
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