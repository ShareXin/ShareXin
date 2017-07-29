use std::env;
use Destination;

#[derive(Debug, Clone, Copy)]
pub struct Language {
    pub service: Destination,
    pub option: u32,
}

pub fn language<'a>(langue: Language) -> &'a str
{
    let service = langue.service;
    //string creates result string, modified later
    let mut string = "";
    //gets user var for language (hopefully?)
    let mut lang = match env::var("LANG") {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to get $LANG. {:?}", e),
    };
    lang = lang.to_lowercase();
    //0 is for image_sent, telling the user their toot/tweet with an image has been sent
    //0 is also for message_sent, telling the user their toot/tweet has been sent
    //2 is for file_saved, telling the user the file is saved
    //3 is for empty, telling the user their message-only toot/tweet was empty
    if langue.option == 0 {
        if lang.contains("fr") {
            if service.mastodon {string = "Envoyé vers Mastodon"; }
            else if service.twitter {string = "Envoyé vers Twitter";}
        }
        else if lang.contains("es") {
            if service.mastodon {string = "Enviado a Mastodon";}
            else if service.twitter {string = "Enviado a Twitter";}
        }
        else if lang.contains("eo") {
            if service.mastodon {string = "Sendita al Mastodon";}
            else if service.twitter {string = "Sendita al Twitter";}
        }
        else if lang.contains("cn") {
            if service.mastodon {string = "它已被发送到Mastodon";}
            else if service.twitter {string = "它已被发送到Twitter";}
        }
        else if lang.contains("tw") {
            if service.mastodon {string = "它已被發送到Mastodon";}
            else if service.twitter {string = "它已被發送到Twitter";}
        }
        else if lang.contains("ja") {
            if service.mastodon {string = "Mastodonに送信";}
            else if service.twitter {string = "Twitterに送信";}
        }
        else if lang.contains("ko") {
            if service.mastodon {string = "Mastodon에 보냈습니다";}
            else if service.twitter {string = "Twitter에 보냈습니다";}
        }
        else if lang.contains("de") {
            if service.mastodon {string = "Auf Mastodon veröffentlicht";}
            else if service.twitter {string = "Auf Twitter veröffentlicht";}
        }
        else {
            if service.mastodon {string = "Sent to Mastodon";}
            else if service.twitter {string = "Sent to Twitter";}
            else if service.imgur {string = "Sent to Imgur";}
        }
    }
    else if langue.option == 2 {
        if lang.contains("fr") {string = "Fichier sauvegardé";}
        else if lang.contains("es") {string = "Archivo guardado";}
        else if lang.contains("eo") {string = "Dosieron konservis";}
        else if lang.contains("cn") {string = "文件已保存";}
        else if lang.contains("tw") {string = "文件已保存";}
        else if lang.contains("ja") {string = "ファイルが保存された";}
        else if lang.contains("ko") {string = "파일이 저장된";}
        else if lang.contains("de") {string = "Datei gespeichert";}
        else {string = "File saved";}
    }
    else if langue.option == 3 {
        if lang.contains("fr") {
            if service.mastodon {string = "Toot vide | N'a pas été envoyé";}
            else if service.twitter {string = "Tweet vide | N'a pas été envoyé";}
        }
        else if lang.contains("es") {
            if service.mastodon {string = "Toot vacío | No fue enviado";}
            else if service.twitter {string = "Tweet vacío | No fue enviado";}
        }
        else if lang.contains("eo") {
            if service.mastodon {string = "Malplena Toot | Ne estis sendita";}
            else if service.twitter {string = "Malplena Tweet | Ne estis sendita";}
        }
        else if lang.contains("cn") {
            if service.mastodon {string = "Toot是空 | 它不会发送到Mastodon";}
            else if service.twitter {string = "Tweet是空 | 它不会发送到Twitter";}
        }
        else if lang.contains("tw") {
            if service.mastodon {string = "Toot是空 | 它不會發送到Mastodon";}
            else if service.twitter {string = "Tweet是空 | 它不會發送到Twitter";}
        }
        else if lang.contains("ja") {
            if service.mastodon {string = "Tootは空です | それはMastodonに送られなかった";}
            else if service.twitter {string = "Tweetは空です | それはTwitterに送られなかった";}
        }
        else if lang.contains("ko") {
            if service.mastodon {string = "Toot 비어 있습니다 | 그것은 Mastodon에 보내지지 않았다";}
            else if service.twitter {string = "Tweet 비어 있습니다 | 그것은 Twitter에 보내지지 않았다";}
        }
        else if lang.contains("de") {
            if service.mastodon {string = "Toot leer | Nicht gesendet";}
            else if service.twitter {string = "Tweet leer | Nicht gesendet";}
        }
        else {
            if service.mastodon {string = "Toot empty | Not sent";}
            else if service.twitter {string = "Tweet empty | Not sent";}
        }
    }
    string
}