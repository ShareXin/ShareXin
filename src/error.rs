/*
    Error Codes:
        0 - Error getting $HOME variable
        1 - Error getting $LANG variable
        2 - Github unreachable
        3 - Imgur unreachable
        4 - t command unavailable
        5 - toot command unavailable
        6 - Error getting $XDG_SESSION_TYPE variable
        7 - Error getting $DESKTOP_SESSION variable
        8 - Error on parsing latest version number
        9 - Unable to open file or webpage
        10 - Error uploading to Imgur
        11 - GTK initialize error
        12 - Unable to get current time
        13 - File not saved
        14 - Gnome-screenshot command unavailable
        15 - Spectacle command unavailable
        16 - Swaygrab command unavailable
        17 - Maim command unavailable
        18 - Unsupported Wayland desktop environment
        19 - Unsupported desktop environment
        20 - Xdotool command unavailable
        21 - Feh command unavailable
        22 - ImageMagick unavailable
        23 - Slop command unavailable
        24 - Unable to read file
        25 - Folder exists
        26 - Unable to send to Mastodon
        27 - Unable to send to Twitter
        28 - Unable to show notification
        29 - convert command unavailable
*/

use language;

pub fn message<'a>(code: usize) -> &'a str {
    let _lang = language::locale();
    let lang = &_lang.to_lowercase();

    if code == 1 {
        return "Error getting $LANG variable";
    } else if code == 0 {
        if lang.contains("fr") {
            return "Erreur lors de la réception de la variable $HOME";
        } else if lang.contains("es") {
            return "Error al obtener la variable $HOME";
        } else if lang.contains("eo") {
            return "Eraro ricevanta la $HOME variablon";
        } else if lang.contains("cn") {
            return "获取$HOME变量时发生错误";
        } else if lang.contains("tw") {
            return "獲取$HOME變量時發生錯誤";
        } else if lang.contains("ja") {
            return "$HOME変数を取得中にエラーが発生しました";
        } else if lang.contains("ko") {
            return "$HOME 변수를 검색하는 동안 오류가 발생했습니다";
        } else if lang.contains("de") {
            return "Fehler beim lesen der $HOME Variable";
        } else {
            return "Error getting $HOME variable";
        }
    } else if code == 2 {
        if lang.contains("fr") {
            return "Github inaccessible";
        } else if lang.contains("es") {
            return "Github inalcanzable";
        } else if lang.contains("eo") {
            return "Github neŝanĝebla";
        } else if lang.contains("cn") {
            return "我不能到达Github";
        } else if lang.contains("tw") {
            return "我不能到達Github";
        } else if lang.contains("ja") {
            return "Githubに到達できない";
        } else if lang.contains("ko") {
            return "Github에 도달 할 수없는";
        } else if lang.contains("de") {
            return "Github unerreichbar";
        } else {
            return "Github unreachable";
        }
    } else if code == 3 {
        if lang.contains("fr") {
            return "Imgur inaccessible";
        } else if lang.contains("es") {
            return "Imgur inalcanzable";
        } else if lang.contains("eo") {
            return "Imgur neŝanĝebla";
        } else if lang.contains("cn") {
            return "我无法到达Imgur";
        } else if lang.contains("tw") {
            return "我無法到達Imgur";
        } else if lang.contains("ja") {
            return "Imgurに到達できない";
        } else if lang.contains("ko") {
            return "Imgur에 도달 할 수없는";
        } else if lang.contains("de") {
            return "Imgur unerreichbar";
        } else {
            return "Imgur unreachable";
        }
    } else if code == 4 {
        if lang.contains("fr") {
            return "Commande t indisponible";
        } else if lang.contains("es") {
            return "t comando no disponible";
        } else if lang.contains("eo") {
            return "t komando ne disponebla";
        } else if lang.contains("cn") {
            return "「t」应用程序不可用";
        } else if lang.contains("tw") {
            return "「t」應用程序不可用";
        } else if lang.contains("ja") {
            return "「t」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「t」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "t command nicht verfügbar";
        } else {
            return "t command unavailable";
        }
    } else if code == 5 {
        if lang.contains("fr") {
            return "Commande toot indisponible";
        } else if lang.contains("es") {
            return "toot comando no disponible";
        } else if lang.contains("eo") {
            return "toot komando ne disponebla";
        } else if lang.contains("cn") {
            return "「toot」应用程序不可用";
        } else if lang.contains("tw") {
            return "「toot」應用程序不可用";
        } else if lang.contains("ja") {
            return "「toot」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「toot」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "toot command nicht verfügbar";
        } else {
            return "toot command unavailable";
        }
    } else if code == 6 {
        if lang.contains("fr") {
            return "Erreur lors de la réception de la variable $XDG_SESSION_TYPE";
        } else if lang.contains("es") {
            return "Error al obtener la variable $XDG_SESSION_TYPE";
        } else if lang.contains("eo") {
            return "Eraro ricevanta la $XDG_SESSION_TYPE variablon";
        } else if lang.contains("cn") {
            return "检索$XDG_SESSION_TYPE变量时发生错误";
        } else if lang.contains("tw") {
            return "檢索$XDG_SESSION_TYPE變量時發生錯誤";
        } else if lang.contains("ja") {
            return "$XDG_SESSION_TYPE変数を取得中にエラーが発生しました";
        } else if lang.contains("ko") {
            return "$XDG_SESSION_TYPE 변수를 검색하는 동안 오류가 발생했습니다";
        } else if lang.contains("de") {
            return "Fehler beim lesen der $XDG_SESSION_TYPE Variable";
        } else {
            return "Error getting $XDG_SESSION_TYPE variable";
        }
    } else if code == 7 {
        if lang.contains("fr") {
            return "Erreur lors de la réception de la variable $DESKTOP_SESSION";
        } else if lang.contains("es") {
            return "Error al obtener la variable $DESKTOP_SESSION";
        } else if lang.contains("eo") {
            return "Eraro ricevanta la $DESKTOP_SESSION variablon";
        } else if lang.contains("cn") {
            return "检索$DESKTOP_SESSION变量时发生错误";
        } else if lang.contains("tw") {
            return "檢索$ DESKTOP_SESSION變量時發生錯誤";
        } else if lang.contains("ja") {
            return "$DESKTOP_SESSION変数を取得中にエラーが発生しました";
        } else if lang.contains("ko") {
            return "$DESKTOP_SESSION 변수를 검색하는 동안 오류가 발생했습니다";
        } else if lang.contains("de") {
            return "Fehler beim lesen der $DESKTOP_SESSION Variable";
        } else {
            return "Error getting $DESKTOP_SESSION variable";
        }
    } else if code == 8 {
        if lang.contains("fr") {
            return "Erreur d'analyse de la dernière version";
        } else if lang.contains("es") {
            return "Error al analizar el último número de versión";
        } else if lang.contains("eo") {
            return "Eraro pri analizado de plej nova versio";
        } else if lang.contains("cn") {
            return "解析最新版本号时出现错误";
        } else if lang.contains("tw") {
            return "解析最新版本號時出現錯誤";
        } else if lang.contains("ja") {
            return "最新のバージョン番号を解析中にエラーが発生しました";
        } else if lang.contains("ko") {
            return "최신 버전 번호를 구문 분석하는 동안 오류가 발생했습니다";
        } else if lang.contains("de") {
            return "Fehler beim Parsen der neuesten Versionsnummer";
        } else {
            return "Error on parsing latest version number";
        }
    } else if code == 9 {
        if lang.contains("fr") {
            return "Incapable d'ouvrir le fichier ou la page web";
        } else if lang.contains("es") {
            return "Error al abrir el archivo o la página web";
        } else if lang.contains("eo") {
            return "Ne eblas malfermi dosieron aŭ retpaĝon";
        } else if lang.contains("cn") {
            return "打开文件或网页时出错";
        } else if lang.contains("tw") {
            return "打開文件或網頁時出錯";
        } else if lang.contains("ja") {
            return "ファイルまたはWebページを開く際にエラーが発生しました";
        } else if lang.contains("ko") {
            return "파일 또는 Web 페이지를 열 때 오류가 발생했습니다";
        } else if lang.contains("de") {
            return "Fehler beim Öffnen der Datei oder Webseite";
        } else {
            return "Unable to open file or webpage";
        }
    } else if code == 10 {
        if lang.contains("fr") {
            return "Erreur lors de l'envoi vers Imgur";
        } else if lang.contains("es") {
            return "Error al subir a Imgur";
        } else if lang.contains("eo") {
            return "Eraro alŝuta al Imgur";
        } else if lang.contains("cn") {
            return "上传到Imgur时发生错误";
        } else if lang.contains("tw") {
            return "上傳到Imgur時發生錯誤";
        } else if lang.contains("ja") {
            return "Imgurへのアップロード中にエラーが発生しました";
        } else if lang.contains("ko") {
            return "Imgur에 업로드하는 중에 오류가 발생했습니다";
        } else if lang.contains("de") {
            return "Fehler beim Uploaden auf Imgur";
        } else {
            return "Error uploading to Imgur";
        }
    } else if code == 11 {
        if lang.contains("fr") {
            return "Erreur d'initialisation GTK";
        } else if lang.contains("es") {
            return "Error de inicialización de GTK";
        } else if lang.contains("eo") {
            return "GTK-komenca eraro";
        } else if lang.contains("cn") {
            return "GTK初始化错误";
        } else if lang.contains("tw") {
            return "GTK初始化錯誤";
        } else if lang.contains("ja") {
            return "GTK初期化エラー";
        } else if lang.contains("ko") {
            return "GTK 초기화 오류";
        } else if lang.contains("de") {
            return "GTK Initialisierungsfehler";
        } else {
            return "GTK initialize error";
        }
    } else if code == 12 {
        if lang.contains("fr") {
            return "L'heure locale n'est pas disponible";
        } else if lang.contains("es") {
            return "Hora local no disponible";
        } else if lang.contains("eo") {
            return "Ne eblas akiri aktualan tempon";
        } else if lang.contains("cn") {
            return "当地时间不可用";
        } else if lang.contains("tw") {
            return "當地時間不可用";
        } else if lang.contains("ja") {
            return "現地時間は利用できません";
        } else if lang.contains("ko") {
            return "현지 시간은 사용할 수 없습니다";
        } else if lang.contains("de") {
            return "Lokale Zeit nicht verfügbar";
        } else {
            return "Unable to get current time";
        }
    } else if code == 13 {
        if lang.contains("fr") {
            return "Le fichier n'est pas sauvegardé";
        } else if lang.contains("es") {
            return "El archivo no se ha guardado";
        } else if lang.contains("eo") {
            return "Dosiero ne savis";
        } else if lang.contains("cn") {
            return "该文件未保存";
        } else if lang.contains("tw") {
            return "該文件未保存";
        } else if lang.contains("ja") {
            return "ファイルは保存されませんでした";
        } else if lang.contains("ko") {
            return "파일이 저장되지 않았습니다";
        } else if lang.contains("de") {
            return "Datei nicht gespeichert";
        } else {
            return "File not saved";
        }
    } else if code == 14 {
        if lang.contains("fr") {
            return "Commande gnome-screenshot indisponible";
        } else if lang.contains("es") {
            return "Comando gnome-screenshot no disponible";
        } else if lang.contains("eo") {
            return "gnome-screenshot ne ekzistas komando";
        } else if lang.contains("cn") {
            return "「gnome-screenshot」应用程序不可用";
        } else if lang.contains("tw") {
            return "「gnome-screenshot」應用程序不可用";
        } else if lang.contains("ja") {
            return "「gnome-screenshot」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「gnome-screenshot」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "gnome-screenshot command nicht verfügbar";
        } else {
            return "gnome-screenshot command unavailable";
        }
    } else if code == 15 {
        if lang.contains("fr") {
            return "Commande spectacle indisponible";
        } else if lang.contains("es") {
            return "Comando spectacle no disponible";
        } else if lang.contains("eo") {
            return "spectacle ne ekzistas komando";
        } else if lang.contains("cn") {
            return "「spectacle」应用程序不可用";
        } else if lang.contains("tw") {
            return "「spectacle」應用程序不可用";
        } else if lang.contains("ja") {
            return "「spectacle」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「spectacle」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "spectacle command nicht verfügbar";
        } else {
            return "spectacle command unavailable";
        }
    } else if code == 16 {
        if lang.contains("fr") {
            return "Commande swaygrab indisponible";
        } else if lang.contains("es") {
            return "Comando swaygrab no disponible";
        } else if lang.contains("eo") {
            return "swaygrab ne ekzistas komando";
        } else if lang.contains("cn") {
            return "「swaygrab」应用程序不可用";
        } else if lang.contains("tw") {
            return "「swaygrab」應用程序不可用";
        } else if lang.contains("ja") {
            return "「swaygrab」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「swaygrab」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "swaygrab command nicht verfügbar";
        } else {
            return "swaygrab command unavailable";
        }
    } else if code == 17 {
        if lang.contains("fr") {
            return "Commande maim indisponible";
        } else if lang.contains("es") {
            return "Comando maim no disponible";
        } else if lang.contains("eo") {
            return "maim ne ekzistas komando";
        } else if lang.contains("cn") {
            return "「maim」应用程序不可用";
        } else if lang.contains("tw") {
            return "「maim」應用程序不可用";
        } else if lang.contains("ja") {
            return "「maim」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「maim」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "maim command nicht verfügbar";
        } else {
            return "maim command unavailable";
        }
    } else if code == 18 {
        if lang.contains("fr") {
            return "L'environnement de bureau Wayland n'est pas pris en charge";
        } else if lang.contains("es") {
            return "Wayland Entorno de escritorio no compatible";
        } else if lang.contains("eo") {
            return "Wayland labortabla medio ne estas subtenata";
        } else if lang.contains("cn") {
            return "您的Wayland桌面环境不受支持";
        } else if lang.contains("tw") {
            return "您的Wayland桌面環境不受支持";
        } else if lang.contains("ja") {
            return "君のWaylandデスクトップ環境はサポートされていません";
        } else if lang.contains("ko") {
            return "너의 Wayland 데스크탑 환경은 지원하지 않습니다";
;
        } else if lang.contains("de") {
            return "Wayland Desktopumgebung nicht unterstützt";
        } else {
            return "Unsupported Wayland desktop environment";
        }
    } else if code == 19 {
        if lang.contains("fr") {
            return "L'environnement de bureau n'est pas pris en charge";
        } else if lang.contains("es") {
            return "Entorno de escritorio no compatible";
        } else if lang.contains("eo") {
            return "Labortabla medio ne estas subtenata";
        } else if lang.contains("cn") {
            return "您的桌面环境不受支持";
        } else if lang.contains("tw") {
            return "您的桌面環境不受支持";
        } else if lang.contains("ja") {
            return "君のデスクトップ環境はサポートされていません";
        } else if lang.contains("ko") {
            return "너의 데스크탑 환경은 지원하지 않습니다";
        } else if lang.contains("de") {
            return "Desktopumgebung nicht unterstützt";
        } else {
            return "Unsupported desktop environment";
        }
    } else if code == 20 {
        if lang.contains("fr") {
            return "Commande xdotool indisponible";
        } else if lang.contains("es") {
            return "Comando xdotool no disponible";
        } else if lang.contains("eo") {
            return "xdotool ne ekzistas komando";
        } else if lang.contains("cn") {
            return "「xdotool」应用程序不可用";
        } else if lang.contains("tw") {
            return "「xdotool」應用程序不可用";
        } else if lang.contains("ja") {
            return "「xdotool」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「xdotool」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "xdotool command nicht verfügbar";
        } else {
            return "xdotool command unavailable";
        }
    } else if code == 21 {
        if lang.contains("fr") {
            return "Commande feh indisponible";
        } else if lang.contains("es") {
            return "Comando feh no disponible";
        } else if lang.contains("eo") {
            return "feh ne ekzistas komando";
        } else if lang.contains("cn") {
            return "「feh」应用程序不可用";
        } else if lang.contains("tw") {
            return "「feh」應用程序不可用";
        } else if lang.contains("ja") {
            return "「feh」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「feh」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "feh command nicht verfügbar";
        } else {
            return "feh command unavailable";
        }
    } else if code == 22 {
        if lang.contains("fr") {
            return "ImageMagick indisponible";
        } else if lang.contains("es") {
            return "ImageMagick no disponible";
        } else if lang.contains("eo") {
            return "ImageMagick ne ekzistas komando";
        } else if lang.contains("cn") {
            return "「ImageMagick」应用程序不可用";
        } else if lang.contains("tw") {
            return "「ImageMagick」應用程序不可用";
        } else if lang.contains("ja") {
            return "「ImageMagick」アプリが利用できない";
        } else if lang.contains("ko") {
            return "「ImageMagick」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "ImageMagick nicht verfügbar";
        } else {
            return "ImageMagick unavailable";
        }
    } else if code == 23 {
        if lang.contains("fr") {
            return "Commande slop indisponible";
        } else if lang.contains("es") {
            return "Comando slop no disponible";
        } else if lang.contains("eo") {
            return "slop ne ekzistas komando";
        } else if lang.contains("cn") {
            return "「slop」应用程序不可用";
        } else if lang.contains("tw") {
            return "「slop」應用程序不可用";
        } else if lang.contains("ja") {
            return "「slop」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「slop」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "slop command nicht verfügbar";
        } else {
            return "slop command unavailable";
        }
    } else if code == 24 {
        if lang.contains("fr") {
            return "Fichier non lisible";
        } else if lang.contains("es") {
            return "Archivo no legible";
        } else if lang.contains("eo") {
            return "Ne eblas legi dosieron";
        } else if lang.contains("cn") {
            return "我无法读取文件";
        } else if lang.contains("tw") {
            return "我無法讀取文件";
        } else if lang.contains("ja") {
            return "ファイルが読めない";
        } else if lang.contains("ko") {
            return "파일을 읽을 수없는";
        } else if lang.contains("de") {
            return "Datei nicht lesbar";
        } else {
            return "Unable to read file";
        }
    } else if code == 25 {
        if lang.contains("fr") {
            return "Le dossier existe déjà";
        } else if lang.contains("es") {
            return "La carpeta ya existe";
        } else if lang.contains("eo") {
            return "La dosierujo jam ekzistas";
        } else if lang.contains("cn") {
            return "文件夹已存在";
        } else if lang.contains("tw") {
            return "文件夾已存在";
        } else if lang.contains("ja") {
            return "フォルダは既に存在します";
        } else if lang.contains("ko") {
            return "폴더는 이미 존재합니다";
        } else if lang.contains("de") {
            return "Ordner existiert bereits";
        } else {
            return "Folder exists";
        }
    } else if code == 26 {
        if lang.contains("fr") {
            return "Impossible de publier sur Mastodon";
        } else if lang.contains("es") {
            return "Error al publicar al Mastodon";
        } else if lang.contains("eo") {
            return "Malsukcesis eldoni al Mastodon";
        } else if lang.contains("cn") {
            return "无法上传到Mastodon";
        } else if lang.contains("tw") {
            return "無法上傳到Mastodon";
        } else if lang.contains("ja") {
            return "Mastodonにアップロードできません";
        } else if lang.contains("ko") {
            return "Mastodon에 업로드 할 수 없습니다";
        } else if lang.contains("de") {
            return "Fehler beim veröffentlichen auf Mastodon";
        } else {
            return "Unable to send to Mastodon";
        }
    } else if code == 27 {
        if lang.contains("fr") {
            return "Impossible de publier sur Twitter";
        } else if lang.contains("es") {
            return "Error al publicar al Twitter";
        } else if lang.contains("eo") {
            return "Malsukcesis eldoni al Twitter";
        } else if lang.contains("cn") {
            return "无法上传到Twitter";
        } else if lang.contains("tw") {
            return "無法上傳到Twitter";
        } else if lang.contains("ja") {
            return "Twitterにアップロードできません";
        } else if lang.contains("ko") {
            return "Twitter에 업로드 할 수 없습니다";
        } else if lang.contains("de") {
            return "Fehler beim veröffentlichen auf Twitter";
        } else {
            return "Unable to send to Twitter";
        }
    } else if code == 28 {
        if lang.contains("fr") {
            return "Impossible d'afficher une notification";
        } else if lang.contains("es") {
            return "No se puede mostrar la notificación";
        } else if lang.contains("eo") {
            return "Ne eblas montri sciigon";
        } else if lang.contains("cn") {
            return "我无法显示通知";
        } else if lang.contains("tw") {
            return "我無法顯示通知";
        } else if lang.contains("ja") {
            return "通知を表示できません";
        } else if lang.contains("ko") {
            return "통지를 표시 할 수 없습니다";
        } else if lang.contains("de") {
            return "Kann keine Benachrichtigung anzeigen";
        } else {
            return "Unable to show notification";
        }
    } else if code == 29 {
        if lang.contains("fr") {
            return "Commande convert indisponible";
        } else if lang.contains("es") {
            return "convert comando no disponible";
        } else if lang.contains("eo") {
            return "convert komando ne disponebla";
        } else if lang.contains("cn") {
            return "「convert」应用程序不可用";
        } else if lang.contains("tw") {
            return "「convert」應用程序不可用";
        } else if lang.contains("ja") {
            return "「convert」アプリケーションが利用できない";
        } else if lang.contains("ko") {
            return "「convert」응용 프로그램을 사용할 수없는";
        } else if lang.contains("de") {
            return "convert command nicht verfügbar";
        } else {
            return "convert command unavailable";
        }
    }

    return "";
}
