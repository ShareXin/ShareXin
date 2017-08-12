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

use std::env;

pub fn message(code: usize) -> String {

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
