#![allow(unused_variables)]
use std::env;
use Destination;
use VERSION;
use language;
use SHAREXIN;
use open;
use std::*;
use curl::easy::Easy;

static APP: &'static str = "sharexin ";

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
            eprintln!("{}", error(2));
            String::from("en_US.utf8").to_lowercase()
        }
    }
}

pub fn notification<'a>(langue: Language) -> &'a str {

    let lang = locale();

    // 0 is for image_sent, telling the user their toot/tweet with an image has been sent
    // 0 is also for message_sent, telling the user their toot/tweet has been sent
    // 2 is for file_saved, telling the user the file is saved
    // 3 is for empty, telling the user their message-only toot/tweet was empty
    // 4 is if the tweet and toot did not send

    if langue.option == 0 {
        if lang.contains("fr") {
            if langue.service.mastodon {
                return "Envoyé vers Mastodon";
            } else if langue.service.twitter {
                return "Envoyé vers Twitter";
            } else if langue.service.imgur {
                return "Envoyé vers Imgur";
            }
        } else if lang.contains("es") {
            if langue.service.mastodon {
                return "Enviado a Mastodon";
            } else if langue.service.twitter {
                return "Enviado a Twitter";
            } else if langue.service.imgur {
                return "Enviado a Imgur";
            }
        } else if lang.contains("eo") {
            if langue.service.mastodon {
                return "Sendita al Mastodon";
            } else if langue.service.twitter {
                return "Sendita al Twitter";
            } else if langue.service.imgur {
                return "Sendita al Imgur";
            }
        } else if lang.contains("cn") {
            if langue.service.mastodon {
                return "它已被发送到Mastodon";
            } else if langue.service.twitter {
                return "它已被发送到Twitter";
            } else if langue.service.imgur {
                return "它已被发送到Imgur";
            }
        } else if lang.contains("tw") {
            if langue.service.mastodon {
                return "它已被發送到Mastodon";
            } else if langue.service.twitter {
                return "它已被發送到Twitter";
            } else if langue.service.imgur {
                return "它已被發送到Imgur";
            }
        } else if lang.contains("ja") {
            if langue.service.mastodon {
                return "Mastodonに送信";
            } else if langue.service.twitter {
                return "Twitterに送信";
            } else if langue.service.imgur {
                return "Imgurに送信";
            }
        } else if lang.contains("ko") {
            if langue.service.mastodon {
                return "Mastodon에 보냈습니다";
            } else if langue.service.twitter {
                return "Twitter에 보냈습니다";
            } else if langue.service.imgur {
                return "Imgur에 보냈습니다";
            }
        } else if lang.contains("de") {
            if langue.service.mastodon {
                return "Auf Mastodon veröffentlicht";
            } else if langue.service.twitter {
                return "Auf Twitter veröffentlicht";
            } else if langue.service.imgur {
                return "Auf Imgur veröffentlicht";
            }
        } else {
            if langue.service.mastodon {
                return "Sent to Mastodon";
            } else if langue.service.twitter {
                return "Sent to Twitter";
            } else if langue.service.imgur {
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
            if langue.service.mastodon {
                return "Toot vide | N'a pas été envoyé";
            } else if langue.service.twitter {
                return "Tweet vide | N'a pas été envoyé";
            }
        } else if lang.contains("es") {
            if langue.service.mastodon {
                return "Toot vacío | No fue enviado";
            } else if langue.service.twitter {
                return "Tweet vacío | No fue enviado";
            }
        } else if lang.contains("eo") {
            if langue.service.mastodon {
                return "Malplena Toot | Ne estis sendita";
            } else if langue.service.twitter {
                return "Malplena Tweet | Ne estis sendita";
            }
        } else if lang.contains("cn") {
            if langue.service.mastodon {
                return "Toot是空 | 它不会发送到Mastodon";
            } else if langue.service.twitter {
                return "Tweet是空 | 它不会发送到Twitter";
            }
        } else if lang.contains("tw") {
            if langue.service.mastodon {
                return "Toot是空 | 它不會發送到Mastodon";
            } else if langue.service.twitter {
                return "Tweet是空 | 它不會發送到Twitter";
            }
        } else if lang.contains("ja") {
            if langue.service.mastodon {
                return "Tootは空です | それはMastodonに送られなかった";
            } else if langue.service.twitter {
                return "Tweetは空です | それはTwitterに送られなかった";
            }
        } else if lang.contains("ko") {
            if langue.service.mastodon {
                return "Toot 비어 있습니다 | 그것은 Mastodon에 보내지지 않았다";
            } else if langue.service.twitter {
                return "Tweet 비어 있습니다 | 그것은 Twitter에 보내지지 않았다";
            }
        } else if lang.contains("de") {
            if langue.service.mastodon {
                return "Toot leer | Nicht gesendet";
            } else if langue.service.twitter {
                return "Tweet leer | Nicht gesendet";
            }
        } else {
            if langue.service.mastodon {
                return "Toot empty | Not sent";
            } else if langue.service.twitter {
                return "Tweet empty | Not sent";
            }
        }
    } else if langue.option == 4 {
        if lang.contains("fr") {
            if langue.service.mastodon {
                return "Toot n'a pas été envoyé";
            } else if langue.service.twitter {
                return "Tweet n'a pas été envoyé";
            }
        } else if lang.contains("es") {
            if langue.service.mastodon {
                return "Toot no fue enviado";
            } else if langue.service.twitter {
                return "Tweet no fue enviado";
            }
        } else if lang.contains("eo") {
            if langue.service.mastodon {
                return "Toot ne estis sendita";
            } else if langue.service.twitter {
                return "Tweet ne estis sendita";
            }
        } else if lang.contains("cn") {
            if langue.service.mastodon {
                return "Toot没有发送";
            } else if langue.service.twitter {
                return "Tweet没有发送";
            }
        } else if lang.contains("tw") {
            if langue.service.mastodon {
                return "Toot沒有發送";
            } else if langue.service.twitter {
                return "Tweet沒有發送";
            }
        } else if lang.contains("ja") {
            if langue.service.mastodon {
                return "Tootが送られなかった";
            } else if langue.service.twitter {
                return "Tweetが送られなかった";
            }
        } else if lang.contains("ko") {
            if langue.service.mastodon {
                return "Toot이 보내지지 않았다";
            } else if langue.service.twitter {
                return "Tweet이 보내지지 않았다";
            }
        } else if lang.contains("de") {
            if langue.service.mastodon {
                return "Toot wurde nicht gesendet";
            } else if langue.service.twitter {
                return "Tweet wurde nicht gesendet";
            }
        } else {
            if langue.service.mastodon {
                return "Toot not sent";
            } else if langue.service.twitter {
                return "Tweet not sent";
            }
        }
    }
    return "";
}

pub fn upgrade() {
    let mut dst = Vec::new();
    let mut latest = Easy::new();

    // file made to check version number
    latest
        .url(
            "https://raw.githubusercontent.com/thebitstick/ShareXin/master/version",
        )
        .unwrap();
    let mut transfer = latest.transfer();
    transfer
        .write_function(|data| {
            dst.extend_from_slice(data);
            let mut latest_utf = String::from_utf8(dst.clone()).unwrap();
            while latest_utf.ends_with("\n") {
                let len = latest_utf.len();
                let new_len = len.saturating_sub("\n".len());
                latest_utf.truncate(new_len);
            }
            let current_version: usize = match str::replace(VERSION, ".", "").parse::<usize>() {
                Ok(ok) => ok,
                Err(_) => {
                    eprintln!("{}", error(18));
                    process::exit(1)
                }
            };
            let latest_version: usize = match str::replace(&latest_utf, ".", "").parse::<usize>() {
                Ok(ok) => ok,
                Err(_) => {
                    eprintln!("{}", error(18));
                    process::exit(1)
                }
            };
            println!(
                "{}",
                check_update(latest_version, current_version, latest_utf)
            );
            Ok(data.len())
        })
        .unwrap();

    match transfer.perform() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error(16));
            process::exit(1)
        }
    };
}

fn check_update(latest_version: usize, current_version: usize, latest_utf: String) -> String {

    let lang = language::locale();

    let mut installed = String::new();
    let mut latest = String::new();
    let mut update_state = String::new();
    if lang.contains("fr") {
        installed.push_str("Version installée");
        latest.push_str("Dernière version");
        if latest_version > current_version {
            update_state.push_str("Vous n'êtes pas à jour!");
        } else if latest_version < current_version {
            update_state.push_str("Vous êtes trop à jour!");
        } else if latest_version == current_version {
            update_state.push_str("Vous êtes à jour!");
        }
    } else if lang.contains("es") {
        installed.push_str("Versión instalada");
        latest.push_str("Ultima versión");
        if latest_version > current_version {
            update_state.push_str("¡Estás fuera de fecha!");
        } else if latest_version < current_version {
            update_state.push_str("Usted está demasiado al día!");
        } else if latest_version == current_version {
            update_state.push_str("¡Estas actualizado!");
        }
    } else if lang.contains("eo") {
        installed.push_str("Instalita versio");
        latest.push_str("Plej lasta versio");
        if latest_version > current_version {
            update_state.push_str("Vi estas eksterdata!");
        } else if latest_version < current_version {
            update_state.push_str("Vi estas tro ĝisdata!");
        } else if latest_version == current_version {
            update_state.push_str("Vi estas ĝisdatigita!");
        }
    } else if lang.contains("cn") {
        installed.push_str("已安装版本");
        latest.push_str("最新版本");
        if latest_version > current_version {
            update_state.push_str("你已经过时了！");
        } else if latest_version < current_version {
            update_state.push_str("你也是最新的！");
        } else if latest_version == current_version {
            update_state.push_str("你是最新的！");
        }
    } else if lang.contains("tw") {
        installed.push_str("已安裝版本");
        latest.push_str("最新版本");
        if latest_version > current_version {
            update_state.push_str("你已經過時了！");
        } else if latest_version < current_version {
            update_state.push_str("你也是最新的！");
        } else if latest_version == current_version {
            update_state.push_str("你是最新的！");
        }
    } else if lang.contains("ja") {
        installed.push_str("インストールされたバージョン");
        latest.push_str("最新バージョン");
        if latest_version > current_version {
            update_state.push_str("あなたは時代遅れです！");
        } else if latest_version < current_version {
            update_state.push_str("あなたはあまりにも最新です！");
        } else if latest_version == current_version {
            update_state.push_str("あなたは最新です！");
        }
    } else if lang.contains("ko") {
        installed.push_str("설치된 버전");
        latest.push_str("최신 버전");
        if latest_version > current_version {
            update_state.push_str("구식입니다!");
        } else if latest_version < current_version {
            update_state.push_str("너는 너무 최신이야!");
        } else if latest_version == current_version {
            update_state.push_str("당신은 최신입니다!");
        }
    } else if lang.contains("de") {
        installed.push_str("Installierte Version");
        latest.push_str("Letzte Version");
        if latest_version > current_version {
            update_state.push_str("Sie sind veraltet!");
        } else if latest_version < current_version {
            update_state.push_str("Sie sind zu auf dem Laufenden!");
        } else if latest_version == current_version {
            update_state.push_str("Sie sind auf dem Laufenden!");
        }
    } else {
        installed.push_str("Installed version");
        latest.push_str("Latest Version");
        if latest_version > current_version {
            update_state.push_str("You are out-of-date!");
        } else if latest_version < current_version {
            update_state.push_str("You are too up-to-date!");
        } else if latest_version == current_version {
            update_state.push_str("You are up-to-date!");
        }
    }

    if latest_version > current_version {
        open_update();
        return format!(
            "{}: {}\n{}: {}\n{}",
            &installed,
            VERSION,
            &latest,
            &latest_utf,
            &update_state
        );
    } else if latest_version < current_version {
        return format!(
            "{}: {}\n{}: {}\n{}",
            &installed,
            VERSION,
            &latest,
            &latest_utf,
            &update_state
        );
    } else if latest_version == current_version {
        return format!(
            "{}: {}\n{}: {}\n{}",
            &installed,
            VERSION,
            &latest,
            &latest_utf,
            &update_state
        );
    }
    return String::new();
}

fn open_update() {
    match open::that(SHAREXIN) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error(19));
            return;
        }
    };

    let lang = language::locale();

    let upgrade_fr = String::from(
        "
Vérifiez les nouvelles mises à jour à l'adresse suivante: ",
    );
    let upgrade_es = String::from(
        "
Busque nuevas actualizaciones en: ",
    );
    let upgrade_eo = String::from(
        "
Kontrolu por novaj ĝisdatigoj ĉe: ",
    );
    let upgrade_cn = String::from(
        "
要检查是否有新的更新：\n",
    );
    let upgrade_tw = String::from(
        "
要檢查是否有新的更新：\n",
    );
    let upgrade_ja = String::from(
        "
新しいアップデートを確認する：\n",
    );
    let upgrade_ko = String::from(
        "
새로운 업데이트 확인 :\n",
    );
    let upgrade_de = String::from(
        "
Überprüfen Sie nach neuen Updates unter: ",
    );
    let upgrade = String::from(
        "
Check for new updates at: ",
    );

    if lang.contains("fr") {
        println!("{}{}", upgrade_fr, SHAREXIN);
    } else if lang.contains("es") {
        println!("{}{}", upgrade_es, SHAREXIN);
    } else if lang.contains("eo") {
        println!("{}{}", upgrade_eo, SHAREXIN);
    } else if lang.contains("cn") {
        println!("{}{}", upgrade_cn, SHAREXIN);
    } else if lang.contains("tw") {
        println!("{}{}", upgrade_tw, SHAREXIN);
    } else if lang.contains("ja") {
        println!("{}{}", upgrade_ja, SHAREXIN);
    } else if lang.contains("ko") {
        println!("{}{}", upgrade_ko, SHAREXIN);
    } else if lang.contains("de") {
        println!("{}{}", upgrade_de, SHAREXIN);
    } else {
        println!("{}{}", upgrade, SHAREXIN);
    }
}

/*
    Error Codes:
        0 - Unknown error
        1 - Error getting $HOME variable
        2 - Error getting $LANG variable
        3 - Error getting $XDG_SESSION_TYPE variable
        4 - Error getting $DESKTOP_SESSION variable
        5 - t command unavailable
        6 - toot command unavailable
        7 - Gnome-screenshot command unavailable
        8 - Spectacle command unavailable
        9 - Swaygrab command unavailable
        10 - Maim command unavailable
        11 - Xdotool command unavailable
        12 - Feh command unavailable
        13 - ImageMagick unavailable
        14 - Slop command unavailable
        15 - convert command unavailable
        16 - Github unreachable
        17 - Imgur unreachable
        18 - Error on parsing latest version number
        19 - Unable to open file or webpage
        20 - Error uploading to Imgur
        21 - Unable to send to Mastodon
        22 - Unable to send to Twitter
        23 - Unable to show notification
        24 - GTK initialize error
        25 - Unable to get current time
        26 - Unsupported Wayland desktop environment
        27 - Unsupported desktop environment
        28 - Unable to read file
        29 - Folder exists
        30 - File not saved
*/

pub fn error(code: usize) -> String {

    let lang = match env::var("LANG") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("Error getting $LANG variable");
            String::from("en_US.utf8").to_lowercase()
        }
    };
    let error_fr = "Erreur";
    let error_es = "Error";
    let error_eo = "Eraro";
    let error_cn = "错误";
    let error_tw = "錯誤";
    let error_ja = "エラー";
    let error_ko = "오류";
    let error_de = "Fehler";
    let error = "Error";

    if code == 0 {
        return format!("Unknown error");
    } else if code == 1 {
        if lang.contains("fr") {
            return format!(
                "{} 1: Erreur lors de la réception de la variable $HOME",
                error_fr
            );
        } else if lang.contains("es") {
            return format!("{} 1: Error al obtener la variable $HOME", error_es);
        } else if lang.contains("eo") {
            return format!("{} 1: Eraro ricevanta la $HOME variablon", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 1: 获取$HOME变量时发生错误", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 1: 獲取$HOME變量時發生錯誤", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 1: $HOME変数を取得中にエラーが発生しました",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 1: $HOME 변수를 검색하는 동안 오류가 발생했습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 1: Fehler beim lesen der $HOME Variable", error_de);
        } else {
            return format!("{} 1: Error getting $HOME variable", error);
        }
    } else if code == 2 {
        return format!("Error getting $LANG variable");
    } else if code == 3 {
        if lang.contains("fr") {
            return format!(
                "{} 3: Erreur lors de la réception de la variable $XDG_SESSION_TYPE",
                error_fr
            );
        } else if lang.contains("es") {
            return format!(
                "{} 3: Error al obtener la variable $XDG_SESSION_TYPE",
                error_es
            );
        } else if lang.contains("eo") {
            return format!(
                "{} 3: Eraro ricevanta la $XDG_SESSION_TYPE variablon",
                error_eo
            );
        } else if lang.contains("cn") {
            return format!(
                "{} 3: 检索$XDG_SESSION_TYPE变量时发生错误",
                error_cn
            );
        } else if lang.contains("tw") {
            return format!(
                "{} 3: 檢索$XDG_SESSION_TYPE變量時發生錯誤",
                error_tw
            );
        } else if lang.contains("ja") {
            return format!(
                "{} 3: $XDG_SESSION_TYPE変数を取得中にエラーが発生しました",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 3: $XDG_SESSION_TYPE 변수를 검색하는 동안 오류가 발생했습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!(
                "{} 3: Fehler beim lesen der $XDG_SESSION_TYPE Variable",
                error_de
            );
        } else {
            return format!("{} 3: Error getting $XDG_SESSION_TYPE variable", error);
        }
    } else if code == 4 {
        if lang.contains("fr") {
            return format!(
                "{} 4: Erreur lors de la réception de la variable $DESKTOP_SESSION",
                error_fr
            );
        } else if lang.contains("es") {
            return format!(
                "{} 4: Error al obtener la variable $DESKTOP_SESSION",
                error_es
            );
        } else if lang.contains("eo") {
            return format!(
                "{} 4: Eraro ricevanta la $DESKTOP_SESSION variablon",
                error_eo
            );
        } else if lang.contains("cn") {
            return format!(
                "{} 4: 检索$DESKTOP_SESSION变量时发生错误",
                error_cn
            );
        } else if lang.contains("tw") {
            return format!(
                "{} 4: 檢索$ DESKTOP_SESSION變量時發生錯誤",
                error_tw
            );
        } else if lang.contains("ja") {
            return format!(
                "{} 4: $DESKTOP_SESSION変数を取得中にエラーが発生しました",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 4: $DESKTOP_SESSION 변수를 검색하는 동안 오류가 발생했습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!(
                "{} 4: Fehler beim lesen der $DESKTOP_SESSION Variable",
                error_de
            );
        } else {
            return format!("{} 4: Error getting $DESKTOP_SESSION variable", error);
        }
    } else if code == 5 {
        if lang.contains("fr") {
            return format!("{} 5: Commande t indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 5: t comando no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 5: t komando ne disponebla", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 5: 「t」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 5: 「t」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 5: 「t」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 5: 「t」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 5: t command nicht verfügbar", error_de);
        } else {
            return format!("{} 5: t command unavailable", error);
        }
    } else if code == 6 {
        if lang.contains("fr") {
            return format!("{} 6: Commande toot indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 6: toot comando no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 6: toot komando ne disponebla", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 6: 「toot」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 6: 「toot」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 6: 「toot」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 6: 「toot」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 6: toot command nicht verfügbar", error_de);
        } else {
            return format!("{} 6: toot command unavailable", error);
        }
    } else if code == 7 {
        if lang.contains("fr") {
            return format!("{} 7: Commande gnome-screenshot indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 7: Comando gnome-screenshot no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 7: gnome-screenshot ne ekzistas komando", error_eo);
        } else if lang.contains("cn") {
            return format!(
                "{} 7: 「gnome-screenshot」应用程序不可用",
                error_cn
            );
        } else if lang.contains("tw") {
            return format!(
                "{} 7: 「gnome-screenshot」應用程序不可用",
                error_tw
            );
        } else if lang.contains("ja") {
            return format!(
                "{} 7: 「gnome-screenshot」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 7: 「gnome-screenshot」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 7: gnome-screenshot command nicht verfügbar", error_de);
        } else {
            return format!("{} 7: gnome-screenshot command unavailable", error);
        }
    } else if code == 8 {
        if lang.contains("fr") {
            return format!("{} 8: Commande spectacle indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 8: Comando spectacle no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 8: spectacle ne ekzistas komando", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 8: 「spectacle」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 8: 「spectacle」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 8: 「spectacle」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 8: 「spectacle」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 8: spectacle command nicht verfügbar", error_de);
        } else {
            return format!("{} 8: spectacle command unavailable", error);
        }
    } else if code == 9 {
        if lang.contains("fr") {
            return format!("{} 9: Commande swaygrab indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 9: Comando swaygrab no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 9: swaygrab ne ekzistas komando", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 9: 「swaygrab」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 9: 「swaygrab」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 9: 「swaygrab」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 9: 「swaygrab」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 9: swaygrab command nicht verfügbar", error_de);
        } else {
            return format!("{} 9: swaygrab command unavailable", error);
        }
    } else if code == 10 {
        if lang.contains("fr") {
            return format!("{} 10: Commande maim indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 10: Comando maim no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 10: maim ne ekzistas komando", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 10: 「maim」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 10: 「maim」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 10: 「maim」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 10: 「maim」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 10: maim command nicht verfügbar", error_de);
        } else {
            return format!("{} 10: maim command unavailable", error);
        }
    } else if code == 11 {
        if lang.contains("fr") {
            return format!("{} 11: Commande xdotool indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 11: Comando xdotool no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 11: xdotool ne ekzistas komando", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 11: 「xdotool」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 11: 「xdotool」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 11: 「xdotool」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 11: 「xdotool」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 11: xdotool command nicht verfügbar", error_de);
        } else {
            return format!("{} 11: xdotool command unavailable", error);
        }
    } else if code == 12 {
        if lang.contains("fr") {
            return format!("{} 12: Commande feh indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 12: Comando feh no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 12: feh ne ekzistas komando", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 12: 「feh」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 12: 「feh」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 12: 「feh」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 12: 「feh」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 12: feh command nicht verfügbar", error_de);
        } else {
            return format!("{} 12: feh command unavailable", error);
        }
    } else if code == 13 {
        if lang.contains("fr") {
            return format!("{} 13: ImageMagick indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 13: ImageMagick no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 13: ImageMagick ne ekzistas komando", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 13: 「ImageMagick」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 13: 「ImageMagick」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 13: 「ImageMagick」アプリが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 13: 「ImageMagick」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 13: ImageMagick nicht verfügbar", error_de);
        } else {
            return format!("{} 13: ImageMagick unavailable", error);
        }
    } else if code == 14 {
        if lang.contains("fr") {
            return format!("{} 14: Commande slop indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 14: Comando slop no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 14: slop ne ekzistas komando", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 14: 「slop」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 14: 「slop」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 14: 「slop」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 14: 「slop」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 14: slop command nicht verfügbar", error_de);
        } else {
            return format!("{} 14: slop command unavailable", error);
        }
    } else if code == 15 {
        if lang.contains("fr") {
            return format!("{} 15: Commande convert indisponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 15: convert comando no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 15: convert komando ne disponebla", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 15: 「convert」应用程序不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 15: 「convert」應用程序不可用", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 15: 「convert」アプリケーションが利用できない",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 15: 「convert」응용 프로그램을 사용할 수없는",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 15: convert command nicht verfügbar", error_de);
        } else {
            return format!("{} 15: convert command unavailable", error);
        }
    } else if code == 16 {
        if lang.contains("fr") {
            return format!("{} 16: Github inaccessible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 16: Github inalcanzable", error_es);
        } else if lang.contains("eo") {
            return format!("{} 16: Github neŝanĝebla", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 16: 我不能到达Github", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 16: 我不能到達Github", error_tw);
        } else if lang.contains("ja") {
            return format!("{} 16: Githubに到達できない", error_ja);
        } else if lang.contains("ko") {
            return format!("{} 16: Github에 도달 할 수없는", error_ko);
        } else if lang.contains("de") {
            return format!("{} 16: Github unerreichbar", error_de);
        } else {
            return format!("{} 16: Github unreachable", error);
        }
    } else if code == 17 {
        if lang.contains("fr") {
            return format!("{} 17: Imgur inaccessible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 17: Imgur inalcanzable", error_es);
        } else if lang.contains("eo") {
            return format!("{} 17: Imgur neŝanĝebla", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 17: 我无法到达Imgur", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 17: 我無法到達Imgur", error_tw);
        } else if lang.contains("ja") {
            return format!("{} 17: Imgurに到達できない", error_ja);
        } else if lang.contains("ko") {
            return format!("{} 17: Imgur에 도달 할 수없는", error_ko);
        } else if lang.contains("de") {
            return format!("{} 17: Imgur unerreichbar", error_de);
        } else {
            return format!("{} 17: Imgur unreachable", error);
        }
    } else if code == 18 {
        if lang.contains("fr") {
            return format!("{} 18: Erreur d'analyse de la dernière version", error_fr);
        } else if lang.contains("es") {
            return format!(
                "{} 18: Error al analizar el último número de versión",
                error_es
            );
        } else if lang.contains("eo") {
            return format!("{} 18: Eraro pri analizado de plej nova versio", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 18: 解析最新版本号时出现错误", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 18: 解析最新版本號時出現錯誤", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 18: 最新のバージョン番号を解析中にエラーが発生しました",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 18: 최신 버전 번호를 구문 분석하는 동안 오류가 발생했습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!(
                "{} 18: Fehler beim Parsen der neuesten Versionsnummer",
                error_de
            );
        } else {
            return format!("{} 18: Error on parsing latest version number", error);
        }
    } else if code == 19 {
        if lang.contains("fr") {
            return format!(
                "{} 19: Incapable d'ouvrir le fichier ou la page web",
                error_fr
            );
        } else if lang.contains("es") {
            return format!(
                "{} 19: Error al abrir el archivo o la página web",
                error_es
            );
        } else if lang.contains("eo") {
            return format!("{} 19: Ne eblas malfermi dosieron aŭ retpaĝon", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 19: 打开文件或网页时出错", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 19: 打開文件或網頁時出錯", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 19: ファイルまたはWebページを開く際にエラーが発生しました",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 19: 파일 또는 Web 페이지를 열 때 오류가 발생했습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!(
                "{} 19: Fehler beim Öffnen der Datei oder Webseite",
                error_de
            );
        } else {
            return format!("{} 19: Unable to open file or webpage", error);
        }
    } else if code == 20 {
        if lang.contains("fr") {
            return format!("{} 20: Erreur lors de l'envoi vers Imgur", error_fr);
        } else if lang.contains("es") {
            return format!("{} 20: Error al subir a Imgur", error_es);
        } else if lang.contains("eo") {
            return format!("{} 20: Eraro alŝuta al Imgur", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 20: 上传到Imgur时发生错误", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 20: 上傳到Imgur時發生錯誤", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 20: Imgurへのアップロード中にエラーが発生しました",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 20: Imgur에 업로드하는 중에 오류가 발생했습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 20: Fehler beim Uploaden auf Imgur", error_de);
        } else {
            return format!("{} 20: Error uploading to Imgur", error);
        }
    } else if code == 21 {
        if lang.contains("fr") {
            return format!("{} 21: Impossible de publier sur Mastodon", error_fr);
        } else if lang.contains("es") {
            return format!("{} 21: Error al publicar al Mastodon", error_es);
        } else if lang.contains("eo") {
            return format!("{} 21: Malsukcesis eldoni al Mastodon", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 21: 无法上传到Mastodon", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 21: 無法上傳到Mastodon", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 21: Mastodonにアップロードできません",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 21: Mastodon에 업로드 할 수 없습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 21: Fehler beim veröffentlichen auf Mastodon", error_de);
        } else {
            return format!("{} 21: Unable to send to Mastodon", error);
        }
    } else if code == 22 {
        if lang.contains("fr") {
            return format!("{} 22: Impossible de publier sur Twitter", error_fr);
        } else if lang.contains("es") {
            return format!("{} 22: Error al publicar al Twitter", error_es);
        } else if lang.contains("eo") {
            return format!("{} 22: Malsukcesis eldoni al Twitter", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 22: 无法上传到Twitter", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 22: 無法上傳到Twitter", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 22: Twitterにアップロードできません",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!("{} 22: Twitter에 업로드 할 수 없습니다", error_ko);
        } else if lang.contains("de") {
            return format!("{} 22: Fehler beim veröffentlichen auf Twitter", error_de);
        } else {
            return format!("{} 22: Unable to send to Twitter", error);
        }
    } else if code == 23 {
        if lang.contains("fr") {
            return format!("{} 23: Impossible d'afficher une notification", error_fr);
        } else if lang.contains("es") {
            return format!("{} 23: No se puede mostrar la notificación", error_es);
        } else if lang.contains("eo") {
            return format!("{} 23: Ne eblas montri sciigon", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 23: 我无法显示通知", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 23: 我無法顯示通知", error_tw);
        } else if lang.contains("ja") {
            return format!("{} 23: 通知を表示できません", error_ja);
        } else if lang.contains("ko") {
            return format!("{} 23: 통지를 표시 할 수 없습니다", error_ko);
        } else if lang.contains("de") {
            return format!("{} 23: Kann keine Benachrichtigung anzeigen", error_de);
        } else {
            return format!("{} 23: Unable to show notification", error);
        }
    } else if code == 24 {
        if lang.contains("fr") {
            return format!("{} 24: Erreur d'initialisation GTK", error_fr);
        } else if lang.contains("es") {
            return format!("{} 24: Error de inicialización de GTK", error_es);
        } else if lang.contains("eo") {
            return format!("{} 24: GTK-komenca eraro", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 24: GTK初始化错误", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 24: GTK初始化錯誤", error_tw);
        } else if lang.contains("ja") {
            return format!("{} 24: GTK初期化エラー", error_ja);
        } else if lang.contains("ko") {
            return format!("{} 24: GTK 초기화 오류", error_ko);
        } else if lang.contains("de") {
            return format!("{} 24: GTK Initialisierungsfehler", error_de);
        } else {
            return format!("{} 24: GTK initialize error", error);
        }
    } else if code == 25 {
        if lang.contains("fr") {
            return format!("{} 25: L'heure locale n'est pas disponible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 25: Hora local no disponible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 25: Ne eblas akiri aktualan tempon", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 25: 当地时间不可用", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 25: 當地時間不可用", error_tw);
        } else if lang.contains("ja") {
            return format!("{} 25: 現地時間は利用できません", error_ja);
        } else if lang.contains("ko") {
            return format!(
                "{} 25: 현지 시간은 사용할 수 없습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 25: Lokale Zeit nicht verfügbar", error_de);
        } else {
            return format!("{} 25: Unable to get current time", error);
        }
    } else if code == 26 {
        if lang.contains("fr") {
            return format!(
                "{} 26: L'environnement de bureau Wayland n'est pas pris en charge",
                error_fr
            );
        } else if lang.contains("es") {
            return format!(
                "{} 26: Wayland Entorno de escritorio no compatible",
                error_es
            );
        } else if lang.contains("eo") {
            return format!(
                "{} 26: Wayland labortabla medio ne estas subtenata",
                error_eo
            );
        } else if lang.contains("cn") {
            return format!("{} 26: 您的Wayland桌面环境不受支持", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 26: 您的Wayland桌面環境不受支持", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 26: 君のWaylandデスクトップ環境はサポートされていません",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 26: 너의 Wayland 데스크탑 환경은 지원하지 않습니다",
                error_ko
            );
;
        } else if lang.contains("de") {
            return format!(
                "{} 26: Wayland Desktopumgebung nicht unterstützt",
                error_de
            );
        } else {
            return format!("{} 26: Unsupported Wayland desktop environment", error);
        }
    } else if code == 27 {
        if lang.contains("fr") {
            return format!(
                "{} 27: L'environnement de bureau n'est pas pris en charge",
                error_fr
            );
        } else if lang.contains("es") {
            return format!("{} 27: Entorno de escritorio no compatible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 27: Labortabla medio ne estas subtenata", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 27: 您的桌面环境不受支持", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 27: 您的桌面環境不受支持", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 27: 君のデスクトップ環境はサポートされていません",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!(
                "{} 27: 너의 데스크탑 환경은 지원하지 않습니다",
                error_ko
            );
        } else if lang.contains("de") {
            return format!("{} 27: Desktopumgebung nicht unterstützt", error_de);
        } else {
            return format!("{} 27: Unsupported desktop environment", error);
        }
    } else if code == 28 {
        if lang.contains("fr") {
            return format!("{} 28: Fichier non lisible", error_fr);
        } else if lang.contains("es") {
            return format!("{} 28: Archivo no legible", error_es);
        } else if lang.contains("eo") {
            return format!("{} 28: Ne eblas legi dosieron", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 28: 我无法读取文件", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 28: 我無法讀取文件", error_tw);
        } else if lang.contains("ja") {
            return format!("{} 28: ファイルが読めない", error_ja);
        } else if lang.contains("ko") {
            return format!("{} 28: 파일을 읽을 수없는", error_ko);
        } else if lang.contains("de") {
            return format!("{} 28: Datei nicht lesbar", error_de);
        } else {
            return format!("{} 28: Unable to read file", error);
        }
    } else if code == 29 {
        if lang.contains("fr") {
            return format!("{} 29: Le dossier existe déjà", error_fr);
        } else if lang.contains("es") {
            return format!("{} 29: La carpeta ya existe", error_es);
        } else if lang.contains("eo") {
            return format!("{} 29: La dosierujo jam ekzistas", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 29: 文件夹已存在", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 29: 文件夾已存在", error_tw);
        } else if lang.contains("ja") {
            return format!("{} 29: フォルダは既に存在します", error_ja);
        } else if lang.contains("ko") {
            return format!("{} 29: 폴더는 이미 존재합니다", error_ko);
        } else if lang.contains("de") {
            return format!("{} 29: Ordner existiert bereits", error_de);
        } else {
            return format!("{} 29: Folder exists", error);
        }
    } else if code == 30 {
        if lang.contains("fr") {
            return format!("{} 30: Le fichier n'est pas sauvegardé", error_fr);
        } else if lang.contains("es") {
            return format!("{} 30: El archivo no se ha guardado", error_es);
        } else if lang.contains("eo") {
            return format!("{} 30: Dosiero ne savis", error_eo);
        } else if lang.contains("cn") {
            return format!("{} 30: 该文件未保存", error_cn);
        } else if lang.contains("tw") {
            return format!("{} 30: 該文件未保存", error_tw);
        } else if lang.contains("ja") {
            return format!(
                "{} 30: ファイルは保存されませんでした",
                error_ja
            );
        } else if lang.contains("ko") {
            return format!("{} 30: 파일이 저장되지 않았습니다", error_ko);
        } else if lang.contains("de") {
            return format!("{} 30: Datei nicht gespeichert", error_de);
        } else {
            return format!("{} 30: File not saved", error);
        }
    }

    return format!("");
}

pub fn help() -> String {

    // snippets of help/usage message in 9 languages, will be formated
    let usage_fr = "Utilisation: ";
    let options_fr = "Options:";
    let help_fr = "Afficher le message d'aide et quitter";
    let version_fr = "Imprimer les informations de la version et quitter";
    let upgrade_fr = "Vérifiez les nouvelles mises à jour";
    let image_fr = "Options d'image:";
    let area_fr = "Capturer une zone de l'écran plutôt que l'écran complet";
    let window_fr = "Capturer la fenêtre active plutôt que l'écran complet";
    let full_fr = "Capturer l'écran complet";
    let file_fr = "Utiliser un fichier";
    let destinations_fr = "Destinations:";
    let toot_fr = "Upload vers Mastodon (en utilisant \"toot\")";
    let tweet_fr = "Upload vers Twitter (en utilisant \"t\")";
    let imgur_fr = "Upload vers Imgur";
    let examples_fr = "Exemples:";

    let usage_es = "Utilización: ";
    let options_es = "Opciones:";
    let help_es = "Mostrar el mensaje de ayuda y sale";
    let version_es = "Imprimir información de la versión y sale";
    let upgrade_es = "Busque nuevas actualizaciones";
    let image_es = "Opciones de imagen:";
    let area_es = "Capturar un área de la pantalla en lugar de la pantalla entera";
    let window_es = "Capturar una ventana en vez de la pantalla entera";
    let full_es = "Capturar la pantalla completa";
    let file_es = "Utilice un archive";
    let destinations_es = "Destinos:";
    let toot_es = "Sube a Mastodon (usando \"toot\")";
    let tweet_es = "Sube a Twitter (usando \"t\")";
    let imgur_es = "Sube a Imgur";
    let examples_es = "Ejemplos:";

    let usage_eo = "Uzo: ";
    let options_eo = "Opcioj:";
    let help_eo = "Montru la helpo mesaĝon kaj eliro";
    let version_eo = "Printversio informoj kaj eliro";
    let upgrade_eo = "Kontrolu por novaj ĝisdatigoj";
    let image_eo = "Opcioj de bildo:";
    let area_eo = "Kapti areon de la ekrano anstataŭ ol la tuta ekrano";
    let window_eo = "Kapti la aktivan fenestron anstataŭ ol la tuta ekrano";
    let full_eo = "Kapti la plena ekrano";
    let file_eo = "Uzu dosieron";
    let destinations_eo = "Celoj:";
    let toot_eo = "Alŝutu al Mastodon (uzante \"toot\")";
    let tweet_eo = "Alŝutu al Twitter (uzante \"t\")";
    let imgur_eo = "Alŝutu al Imgur";
    let examples_eo = "Ekzemploj:";

    let usage_cn = "使用方法: ";
    let options_cn = "选项:";
    let help_cn = "退出标准成功输出输出用法消息。";
    let version_cn = "在成功完成输出版本信息到标准输出。";
    let upgrade_cn = "要检查新的更新。";
    let image_cn = "选项截图:";
    let area_cn = "截取屏幕的一个区域，而不是整个屏幕";
    let window_cn = "截取窗口，而不是整个屏幕";
    let full_cn = "为了让整个屏幕";
    let file_cn = "使用文件";
    let destinations_cn = "目的地:";
    let toot_cn = "上传到Mastodon（使用 「toot」）";
    let tweet_cn = "上传到Twitter（使用 「t」）";
    let imgur_cn = "上传到Imgur";
    let examples_cn = "案件:";

    let usage_tw = "使用方法: ";
    let options_tw = "選項:";
    let help_tw = "退出標準成功輸出輸出用法消息。";
    let version_tw = "在成功完成輸出版本信息到標準輸出。";
    let upgrade_tw = "要檢查新的更新。";
    let image_tw = "選項截圖:";
    let area_tw = "擷取畫面的一個區域而不是整個畫面";
    let window_tw = "擷取單一視窗而不是整個畫面";
    let full_tw = "為了讓整個屏幕";
    let file_tw = "使用文件";
    let destinations_tw = "目的地:";
    let toot_tw = "上傳到Mastodon（使用 「toot」）";
    let tweet_tw = "上傳到Twitter（使用 「t」）";
    let imgur_tw = "上傳到Imgur";
    let examples_tw = "案件:";

    let usage_ja = "使用方法: ";
    let options_ja = "オプション:";
    let help_ja = "標準出力に使用方法のメッセージを出力して正常終了する。";
    let version_ja = "標準出力にバージョン情報を出力して正常終了する。";
    let upgrade_ja = "新しい更新を確認する。";
    let image_ja = "スクリーンショットのオプション:";
    let area_ja = "画面全体ではなく一部を取得する";
    let window_ja = "画面全体ではなくウィンドウ単体を取得する";
    let full_ja = "画面全体を取得する";
    let file_ja = "ファイルを使って";
    let destinations_ja = "行き先:";
    let toot_ja = "マストドンにアップロード（使用して「ｔｏｏｔ」)";
    let tweet_ja = "ツイッターにアップロード（使用して「ｔ」)";
    let imgur_ja = "Imgurにアップロード";
    let examples_ja = "例:";

    let usage_ko = "사용 방법: ";
    let options_ko = "옵션:";
    let help_ko = "표준 출력으로 사용법을 출력하고 정상적으로 종료한다.";
    let version_ko = "표준 출력으로 버전 정보를 출력하고 정상적으로 종료한다.";
    let upgrade_ko = "새로운 업데이트를 확인한다.";
    let image_ko = "스크린 샷 옵션:";
    let area_ko = "전체 화면 대신에 화면의 일정 영역을 찍습니다";
    let window_ko = "전체 화면 대신에 창을 찍습니다";
    let full_ko = "전체 화면을 얻을";
    let file_ko = "파일을 사용하여";
    let destinations_ko = "목적지:";
    let toot_ko = "Mastodon에 업로드 (사용 「toot」)";
    let tweet_ko = "Twitter에 업로드 (사용 「t」)";
    let imgur_ko = "Imgur에 업로드";
    let examples_ko = "예:";

    let usage_de = "Anwendung: ";
    let options_de = "Optionen:";
    let help_de = "Zeige diese Nachricht an und beende";
    let version_de = "Gebe Version aus und beende";
    let upgrade_de = "Auf neue Updates prüfen";
    let image_de = "Bildoptionen:";
    let area_de = "Nur einen Bereich anstatt des gesamten Bildschirms aufnehmen";
    let window_de = "Nur ein Fenster anstatt des gesamten Bildschirms aufnehmen";
    let full_de = "Gesamten Bildschirms aufnehmen";
    let file_de = "Benutze eine Datei";
    let destinations_de = "Reiseziele:";
    let toot_de = "Auf Mastodon veröffentlichen (benutzt \"toot\")";
    let tweet_de = "Auf Twitter veröffentlichen (benutzt \"t\")";
    let imgur_de = "Auf Imgur veröffentlichen";
    let examples_de = "Beispiele:";

    let usage_en = "Usage: ";
    let options_en = "Options:";
    let help_en = "Display this help message and exit";
    let version_en = "Print version info and exit";
    let upgrade_en = "Check for new updates";
    let image_en = "Image Options:";
    let area_en = "Grab an area of the screen instead of the entire screen";
    let window_en = "Grab the current window instead of the entire screen";
    let full_en = "Grab the entire screen";
    let file_en = "Use a file";
    let destinations_en = "Destinations:";
    let toot_en = "Upload to Mastodon (uses \"toot\")";
    let tweet_en = "Upload to Twitter (uses \"t\")";
    let imgur_en = "Upload to Imgur";
    let examples_en = "Examples:";

    let usage_usage = "sharexin <options> [destination] [image options] [FILE]";
    let usage_examples = "  sharexin toot
  sharexin tweet full
  sharexin toot area
  sharexin imgur file [FILE]";

    // templates
    let usage: &str;
    let options: &str;
    let help: &str;
    let version: &str;
    let upgrade: &str;
    let image: &str;
    let area: &str;
    let window: &str;
    let full: &str;
    let file: &str;
    let destinations: &str;
    let toot: &str;
    let tweet: &str;
    let imgur: &str;
    let examples: &str;

    let _lang = language::locale();
    let lang = &_lang.to_lowercase();

    if lang.contains("fr") {
        usage = usage_fr;
        options = options_fr;
        help = help_fr;
        version = version_fr;
        upgrade = upgrade_fr;
        image = image_fr;
        area = area_fr;
        window = window_fr;
        full = full_fr;
        file = file_fr;
        destinations = destinations_fr;
        toot = toot_fr;
        tweet = tweet_fr;
        imgur = imgur_fr;
        examples = examples_fr;
    } else if lang.contains("es") {
        usage = usage_es;
        options = options_es;
        help = help_es;
        version = version_es;
        upgrade = upgrade_es;
        image = image_es;
        area = area_es;
        window = window_es;
        full = full_es;
        file = file_es;
        destinations = destinations_es;
        toot = toot_es;
        tweet = tweet_es;
        imgur = imgur_es;
        examples = examples_es;
    } else if lang.contains("eo") {
        usage = usage_eo;
        options = options_eo;
        help = help_eo;
        version = version_eo;
        upgrade = upgrade_eo;
        image = image_eo;
        area = area_eo;
        window = window_eo;
        full = full_eo;
        file = file_eo;
        destinations = destinations_eo;
        toot = toot_eo;
        tweet = tweet_eo;
        imgur = imgur_eo;
        examples = examples_eo;
    } else if lang.contains("cn") {
        usage = usage_cn;
        options = options_cn;
        help = help_cn;
        version = version_cn;
        upgrade = upgrade_cn;
        image = image_cn;
        area = area_cn;
        window = window_cn;
        full = full_cn;
        file = file_cn;
        destinations = destinations_cn;
        toot = toot_cn;
        tweet = tweet_cn;
        imgur = imgur_cn;
        examples = examples_cn;
    } else if lang.contains("tw") {
        usage = usage_tw;
        options = options_tw;
        help = help_tw;
        version = version_tw;
        upgrade = upgrade_tw;
        image = image_tw;
        area = area_tw;
        window = window_tw;
        full = full_tw;
        file = file_tw;
        destinations = destinations_tw;
        toot = toot_tw;
        tweet = tweet_tw;
        imgur = imgur_tw;
        examples = examples_tw;
    } else if lang.contains("ja") {
        usage = usage_ja;
        options = options_ja;
        help = help_ja;
        version = version_ja;
        upgrade = upgrade_ja;
        image = image_ja;
        area = area_ja;
        window = window_ja;
        full = full_ja;
        file = file_ja;
        destinations = destinations_ja;
        toot = toot_ja;
        tweet = tweet_ja;
        imgur = imgur_ja;
        examples = examples_ja;
    } else if lang.contains("ko") {
        usage = usage_ko;
        options = options_ko;
        help = help_ko;
        version = version_ko;
        upgrade = upgrade_ko;
        image = image_ko;
        area = area_ko;
        window = window_ko;
        full = full_ko;
        file = file_ko;
        destinations = destinations_ko;
        toot = toot_ko;
        tweet = tweet_ko;
        imgur = imgur_ko;
        examples = examples_ko;
    } else if lang.contains("de") {
        usage = usage_de;
        options = options_de;
        help = help_de;
        version = version_de;
        upgrade = upgrade_de;
        image = image_de;
        area = area_de;
        window = window_de;
        full = full_de;
        file = file_de;
        destinations = destinations_de;
        toot = toot_de;
        tweet = tweet_de;
        imgur = imgur_de;
        examples = examples_de;
    } else {
        usage = usage_en;
        options = options_en;
        help = help_en;
        version = version_en;
        upgrade = upgrade_en;
        image = image_en;
        area = area_en;
        window = window_en;
        full = full_en;
        file = file_en;
        destinations = destinations_en;
        toot = toot_en;
        tweet = tweet_en;
        imgur = imgur_en;
        examples = examples_en;
    }

    return format!(
        "{}{}\n{}{}\n\n{}\n  -h, --help\t{}\n  -V, --version\t{}\n  -U, --upgrade\t{}\n
{}\n  area\t\t{}\n  window\t{}\n  full\t\t{}\n  file\t\t{}\n
{}\n  toot\t\t{}\n  tweet\t\t{}\n  imgur\t\t{}\n
{}\n{}",
        APP,
        VERSION,
        usage,
        usage_usage,
        options,
        help,
        version,
        upgrade,
        image,
        area,
        window,
        full,
        file,
        destinations,
        toot,
        tweet,
        imgur,
        examples,
        usage_examples
    );

}
