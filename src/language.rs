#![allow(unused_variables)]
use std::*;
use error;
use Destination;

#[derive(Debug, Clone, Copy)]
pub struct Language {
    service: Destination,
    option: u32,
}

impl Language {
    pub fn new(service: Destination, option: u32) -> Language {
        Language {
            service: service,
            option: option,
        }
    }
}

// uses $LANG to get setting from user, should work without any user modification
pub fn locale() -> String {
    match env::var("LANG") {
        Ok(ok) => ok.to_lowercase(),
        Err(e) => {
            eprintln!("{}", error::message(2));
            String::from("en_US.utf8").to_lowercase()
        }
    }
}

pub fn language<'a>(langue: Language) -> &'a str {
    let service = langue.service;

    let lang = locale();

    // 0 is for image_sent, telling the user their toot/tweet with an image has been sent
    // 0 is also for message_sent, telling the user their toot/tweet has been sent
    // 2 is for file_saved, telling the user the file is saved
    // 3 is for empty, telling the user their message-only toot/tweet was empty
    // 4 is if the tweet and toot did not send

    if langue.option == 0 {
        if lang.contains("fr") {
            if service.mastodon {
                return "Envoyé vers Mastodon";
            } else if service.twitter {
                return "Envoyé vers Twitter";
            } else if service.imgur {
                return "Envoyé vers Imgur";
            }
        } else if lang.contains("es") {
            if service.mastodon {
                return "Enviado a Mastodon";
            } else if service.twitter {
                return "Enviado a Twitter";
            } else if service.imgur {
                return "Enviado a Imgur";
            }
        } else if lang.contains("eo") {
            if service.mastodon {
                return "Sendita al Mastodon";
            } else if service.twitter {
                return "Sendita al Twitter";
            } else if service.imgur {
                return "Sendita al Imgur";
            }
        } else if lang.contains("cn") {
            if service.mastodon {
                return "它已被发送到Mastodon";
            } else if service.twitter {
                return "它已被发送到Twitter";
            } else if service.imgur {
                return "它已被发送到Imgur";
            }
        } else if lang.contains("tw") {
            if service.mastodon {
                return "它已被發送到Mastodon";
            } else if service.twitter {
                return "它已被發送到Twitter";
            } else if service.imgur {
                return "它已被發送到Imgur";
            }
        } else if lang.contains("ja") {
            if service.mastodon {
                return "Mastodonに送信";
            } else if service.twitter {
                return "Twitterに送信";
            } else if service.imgur {
                return "Imgurに送信";
            }
        } else if lang.contains("ko") {
            if service.mastodon {
                return "Mastodon에 보냈습니다";
            } else if service.twitter {
                return "Twitter에 보냈습니다";
            } else if service.imgur {
                return "Imgur에 보냈습니다";
            }
        } else if lang.contains("de") {
            if service.mastodon {
                return "Auf Mastodon veröffentlicht";
            } else if service.twitter {
                return "Auf Twitter veröffentlicht";
            } else if service.imgur {
                return "Auf Imgur veröffentlicht";
            }
        } else {
            if service.mastodon {
                return "Sent to Mastodon";
            } else if service.twitter {
                return "Sent to Twitter";
            } else if service.imgur {
                return "Sent to Imgur";
            }
        }
    } else if langue.option == 2 {
        if lang.contains("fr") {
            return "Fichier sauvegardé";
        } else if lang.contains("es") {
            return "Archivo guardado";
        } else if lang.contains("eo") {
            return "Dosieron konservis";
        } else if lang.contains("cn") {
            return "文件已保存";
        } else if lang.contains("tw") {
            return "文件已保存";
        } else if lang.contains("ja") {
            return "ファイルが保存された";
        } else if lang.contains("ko") {
            return "파일이 저장된";
        } else if lang.contains("de") {
            return "Datei gespeichert";
        } else {
            return "File saved";
        }
    } else if langue.option == 3 {
        if lang.contains("fr") {
            if service.mastodon {
                return "Toot vide | N'a pas été envoyé";
            } else if service.twitter {
                return "Tweet vide | N'a pas été envoyé";
            }
        } else if lang.contains("es") {
            if service.mastodon {
                return "Toot vacío | No fue enviado";
            } else if service.twitter {
                return "Tweet vacío | No fue enviado";
            }
        } else if lang.contains("eo") {
            if service.mastodon {
                return "Malplena Toot | Ne estis sendita";
            } else if service.twitter {
                return "Malplena Tweet | Ne estis sendita";
            }
        } else if lang.contains("cn") {
            if service.mastodon {
                return "Toot是空 | 它不会发送到Mastodon";
            } else if service.twitter {
                return "Tweet是空 | 它不会发送到Twitter";
            }
        } else if lang.contains("tw") {
            if service.mastodon {
                return "Toot是空 | 它不會發送到Mastodon";
            } else if service.twitter {
                return "Tweet是空 | 它不會發送到Twitter";
            }
        } else if lang.contains("ja") {
            if service.mastodon {
                return "Tootは空です | それはMastodonに送られなかった";
            } else if service.twitter {
                return "Tweetは空です | それはTwitterに送られなかった";
            }
        } else if lang.contains("ko") {
            if service.mastodon {
                return "Toot 비어 있습니다 | 그것은 Mastodon에 보내지지 않았다";
            } else if service.twitter {
                return "Tweet 비어 있습니다 | 그것은 Twitter에 보내지지 않았다";
            }
        } else if lang.contains("de") {
            if service.mastodon {
                return "Toot leer | Nicht gesendet";
            } else if service.twitter {
                return "Tweet leer | Nicht gesendet";
            }
        } else {
            if service.mastodon {
                return "Toot empty | Not sent";
            } else if service.twitter {
                return "Tweet empty | Not sent";
            }
        }
    } else if langue.option == 4 {
        if lang.contains("fr") {
            if service.mastodon {
                return "Toot n'a pas été envoyé";
            } else if service.twitter {
                return "Tweet n'a pas été envoyé";
            }
        } else if lang.contains("es") {
            if service.mastodon {
                return "Toot no fue enviado";
            } else if service.twitter {
                return "Tweet no fue enviado";
            }
        } else if lang.contains("eo") {
            if service.mastodon {
                return "Toot ne estis sendita";
            } else if service.twitter {
                return "Tweet ne estis sendita";
            }
        } else if lang.contains("cn") {
            if service.mastodon {
                return "Toot没有发送";
            } else if service.twitter {
                return "Tweet没有发送";
            }
        } else if lang.contains("tw") {
            if service.mastodon {
                return "Toot沒有發送";
            } else if service.twitter {
                return "Tweet沒有發送";
            }
        } else if lang.contains("ja") {
            if service.mastodon {
                return "Tootが送られなかった";
            } else if service.twitter {
                return "Tweetが送られなかった";
            }
        } else if lang.contains("ko") {
            if service.mastodon {
                return "Toot이 보내지지 않았다";
            } else if service.twitter {
                return "Tweet이 보내지지 않았다";
            }
        } else if lang.contains("de") {
            if service.mastodon {
                return "Toot wurde nicht gesendet";
            } else if service.twitter {
                return "Tweet wurde nicht gesendet";
            }
        } else {
            if service.mastodon {
                return "Toot not sent";
            } else if service.twitter {
                return "Tweet not sent";
            }
        }
    }
    return "";
}
