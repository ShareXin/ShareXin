use VERSION;
use SHAREXIN;
use open;
use std::*;
use curl::easy::Easy;
use error;
use language;

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
                    eprintln!("{}", error::message(18));
                    process::exit(1)
                }
            };
            let latest_version: usize = match str::replace(&latest_utf, ".", "").parse::<usize>() {
                Ok(ok) => ok,
                Err(_) => {
                    eprintln!("{}", error::message(18));
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
            eprintln!("{}", error::message(16));
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
            eprintln!("{}", error::message(19));
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
