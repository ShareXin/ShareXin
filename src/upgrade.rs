#![allow(unused_variables)]
use VERSION;
use SHAREXIN;
use open;
use std::*;
use curl::easy::Easy;
use std::env;
use error;

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
                Err(e) => {
                    println!("{}", error::message(8));
                    process::exit(1)
                }
            };
            let latest_version: usize = match str::replace(&latest_utf, ".", "").parse::<usize>() {
                Ok(ok) => ok,
                Err(e) => {
                    println!("{}", error::message(8));
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
        Err(e) => {
            println!("{}", error::message(2));
            process::exit(1)
        }
    };
}

fn check_update(latest_version: usize, current_version: usize, latest_utf: String) -> String {
    let _lang = match env::var("LANG") {
        Ok(ok) => ok,
        Err(e) => {
            println!("{}", error::message(1));
            process::exit(1)
        }
    };
    let lang = &_lang.to_lowercase();

    let mut _return = String::new();

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
        _return.push_str(&installed);
        _return.push_str(": ");
        _return.push_str(VERSION);
        _return.push_str("\n");
        _return.push_str(&latest);
        _return.push_str(": ");
        _return.push_str(&latest_utf);
        _return.push_str("\n");
        _return.push_str(&update_state);
        open_update();
    } else if latest_version < current_version {
        _return.push_str(&installed);
        _return.push_str(": ");
        _return.push_str(VERSION);
        _return.push_str("\n");
        _return.push_str(&latest);
        _return.push_str(": ");
        _return.push_str(&latest_utf);
        _return.push_str("\n");
        _return.push_str(&update_state);
    } else if latest_version == current_version {
        _return.push_str(&installed);
        _return.push_str(": ");
        _return.push_str(VERSION);
        _return.push_str("\n");
        _return.push_str(&latest);
        _return.push_str(": ");
        _return.push_str(&latest_utf);
        _return.push_str("\n");
        _return.push_str(&update_state);
    }
    _return
}

fn open_update() {
    match open::that(SHAREXIN) {
        Ok(ok) => ok,
        Err(e) => {
            println!("{}", error::message(9));
            process::exit(1)
        }
    };

    // checks $LANG variable, should work universally on unix-like systems
    let _lang = match env::var("LANG") {
        Ok(ok) => ok,
        Err(e) => {
            println!("{}", error::message(1));
            process::exit(1)
        }
    };
    let lang = &_lang.to_lowercase();

    let mut upgrade_fr = String::from(
        "
Vérifiez les nouvelles mises à jour à l'adresse suivante: ",
    );
    let mut upgrade_es = String::from(
        "
Busque nuevas actualizaciones en: ",
    );
    let mut upgrade_eo = String::from(
        "
Kontrolu por novaj ĝisdatigoj ĉe: ",
    );
    let mut upgrade_cn = String::from(
        "
要检查是否有新的更新：\n",
    );
    let mut upgrade_tw = String::from(
        "
要檢查是否有新的更新：\n",
    );
    let mut upgrade_ja = String::from(
        "
新しいアップデートを確認する：\n",
    );
    let mut upgrade_ko = String::from(
        "
새로운 업데이트 확인 :\n",
    );
    let mut upgrade_de = String::from(
        "
Überprüfen Sie nach neuen Updates unter: ",
    );
    let mut upgrade = String::from(
        "
Check for new updates at: ",
    );
    upgrade_fr.push_str(SHAREXIN);
    upgrade_es.push_str(SHAREXIN);
    upgrade_eo.push_str(SHAREXIN);
    upgrade_cn.push_str(SHAREXIN);
    upgrade_tw.push_str(SHAREXIN);
    upgrade_ja.push_str(SHAREXIN);
    upgrade_ko.push_str(SHAREXIN);
    upgrade_de.push_str(SHAREXIN);
    upgrade.push_str(SHAREXIN);

    if lang.contains("fr") {
        println!("{}", upgrade_fr);
    } else if lang.contains("es") {
        println!("{}", upgrade_es);
    } else if lang.contains("eo") {
        println!("{}", upgrade_eo);
    } else if lang.contains("cn") {
        println!("{}", upgrade_cn);
    } else if lang.contains("tw") {
        println!("{}", upgrade_tw);
    } else if lang.contains("ja") {
        println!("{}", upgrade_ja);
    } else if lang.contains("ko") {
        println!("{}", upgrade_ko);
    } else if lang.contains("de") {
        println!("{}", upgrade_de);
    } else {
        println!("{}", upgrade);
    }
}
