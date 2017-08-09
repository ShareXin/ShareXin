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
*/

use language;

pub fn message(code: usize) -> String {
    let mut _return = String::new();
    let _lang = language::locale();
    let lang = &_lang.to_lowercase();

    if code == 1 {
        _return.push_str("Error getting $LANG variable");
    } else if code == 0 {
        if lang.contains("fr") {
            _return.push_str("Erreur lors de la réception de la variable $HOME");
        } else if lang.contains("es") {
            _return.push_str("Error al obtener la variable $HOME");
        } else if lang.contains("eo") {
            _return.push_str("Eraro ricevanta la $HOME variablon");
        } else if lang.contains("cn") {
            _return.push_str("获取$HOME变量时发生错误");
        } else if lang.contains("tw") {
            _return.push_str("獲取$HOME變量時發生錯誤");
        } else if lang.contains("ja") {
            _return.push_str("$HOME変数を取得中にエラーが発生しました");
        } else if lang.contains("ko") {
            _return.push_str(
                "$HOME 변수를 검색하는 동안 오류가 발생했습니다",
            );
        } else if lang.contains("de") {
            _return.push_str("Fehler beim lesen der $HOME Variable");
        } else {
            _return.push_str("Error getting $HOME variable");
        }
    } else if code == 2 {
        if lang.contains("fr") {
            _return.push_str("Github inaccessible");
        } else if lang.contains("es") {
            _return.push_str("Github inalcanzable");
        } else if lang.contains("eo") {
            _return.push_str("Github neŝanĝebla");
        } else if lang.contains("cn") {
            _return.push_str("我不能到达Github");
        } else if lang.contains("tw") {
            _return.push_str("我不能到達Github");
        } else if lang.contains("ja") {
            _return.push_str("Githubに到達できない");
        } else if lang.contains("ko") {
            _return.push_str("Github에 도달 할 수없는");
        } else if lang.contains("de") {
            _return.push_str("Github unerreichbar");
        } else {
            _return.push_str("Github unreachable");
        }
    } else if code == 3 {
        if lang.contains("fr") {
            _return.push_str("Imgur inaccessible");
        } else if lang.contains("es") {
            _return.push_str("Imgur inalcanzable");
        } else if lang.contains("eo") {
            _return.push_str("Imgur neŝanĝebla");
        } else if lang.contains("cn") {
            _return.push_str("我无法到达Imgur");
        } else if lang.contains("tw") {
            _return.push_str("我無法到達Imgur");
        } else if lang.contains("ja") {
            _return.push_str("Imgurに到達できない");
        } else if lang.contains("ko") {
            _return.push_str("Imgur에 도달 할 수없는");
        } else if lang.contains("de") {
            _return.push_str("Imgur unerreichbar");
        } else {
            _return.push_str("Imgur unreachable");
        }
    } else if code == 4 {
        if lang.contains("fr") {
            _return.push_str("Commande t indisponible");
        } else if lang.contains("es") {
            _return.push_str("t comando no disponible");
        } else if lang.contains("eo") {
            _return.push_str("t komando ne disponebla");
        } else if lang.contains("cn") {
            _return.push_str("「t」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「t」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str("「t」アプリケーションが利用できない");
        } else if lang.contains("ko") {
            _return.push_str("「t」응용 프로그램을 사용할 수없는");
        } else if lang.contains("de") {
            _return.push_str("t command nicht verfügbar");
        } else {
            _return.push_str("t command unavailable");
        }
    } else if code == 5 {
        if lang.contains("fr") {
            _return.push_str("Commande toot indisponible");
        } else if lang.contains("es") {
            _return.push_str("toot comando no disponible");
        } else if lang.contains("eo") {
            _return.push_str("toot komando ne disponebla");
        } else if lang.contains("cn") {
            _return.push_str("「toot」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「toot」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str("「toot」アプリケーションが利用できない");
        } else if lang.contains("ko") {
            _return.push_str("「toot」응용 프로그램을 사용할 수없는");
        } else if lang.contains("de") {
            _return.push_str("toot command nicht verfügbar");
        } else {
            _return.push_str("toot command unavailable");
        }
    } else if code == 6 {
        if lang.contains("fr") {
            _return.push_str(
                "Erreur lors de la réception de la variable $XDG_SESSION_TYPE",
            );
        } else if lang.contains("es") {
            _return.push_str("Error al obtener la variable $XDG_SESSION_TYPE");
        } else if lang.contains("eo") {
            _return.push_str("Eraro ricevanta la $XDG_SESSION_TYPE variablon");
        } else if lang.contains("cn") {
            _return.push_str("检索$XDG_SESSION_TYPE变量时发生错误");
        } else if lang.contains("tw") {
            _return.push_str("檢索$XDG_SESSION_TYPE變量時發生錯誤");
        } else if lang.contains("ja") {
            _return.push_str(
                "$XDG_SESSION_TYPE変数を取得中にエラーが発生しました",
            );
        } else if lang.contains("ko") {
            _return.push_str(
                "$XDG_SESSION_TYPE 변수를 검색하는 동안 오류가 발생했습니다",
            );
        } else if lang.contains("de") {
            _return.push_str("Fehler beim lesen der $XDG_SESSION_TYPE Variable");
        } else {
            _return.push_str("Error getting $XDG_SESSION_TYPE variable");
        }
    } else if code == 7 {
        if lang.contains("fr") {
            _return.push_str(
                "Erreur lors de la réception de la variable $DESKTOP_SESSION",
            );
        } else if lang.contains("es") {
            _return.push_str("Error al obtener la variable $DESKTOP_SESSION");
        } else if lang.contains("eo") {
            _return.push_str("Eraro ricevanta la $DESKTOP_SESSION variablon");
        } else if lang.contains("cn") {
            _return.push_str("检索$DESKTOP_SESSION变量时发生错误");
        } else if lang.contains("tw") {
            _return.push_str("檢索$ DESKTOP_SESSION變量時發生錯誤");
        } else if lang.contains("ja") {
            _return.push_str(
                "$DESKTOP_SESSION変数を取得中にエラーが発生しました",
            );
        } else if lang.contains("ko") {
            _return.push_str(
                "$DESKTOP_SESSION 변수를 검색하는 동안 오류가 발생했습니다",
            );
        } else if lang.contains("de") {
            _return.push_str("Fehler beim lesen der $DESKTOP_SESSION Variable");
        } else {
            _return.push_str("Error getting $DESKTOP_SESSION variable");
        }
    } else if code == 8 {
        if lang.contains("fr") {
            _return.push_str("Erreur d'analyse de la dernière version");
        } else if lang.contains("es") {
            _return.push_str("Error al analizar el último número de versión");
        } else if lang.contains("eo") {
            _return.push_str("Eraro pri analizado de plej nova versio");
        } else if lang.contains("cn") {
            _return.push_str("解析最新版本号时出现错误");
        } else if lang.contains("tw") {
            _return.push_str("解析最新版本號時出現錯誤");
        } else if lang.contains("ja") {
            _return.push_str(
                "最新のバージョン番号を解析中にエラーが発生しました",
            );
        } else if lang.contains("ko") {
            _return.push_str(
                "최신 버전 번호를 구문 분석하는 동안 오류가 발생했습니다",
            );
        } else if lang.contains("de") {
            _return.push_str("Fehler beim Parsen der neuesten Versionsnummer");
        } else {
            _return.push_str("Error on parsing latest version number");
        }
    } else if code == 9 {
        if lang.contains("fr") {
            _return.push_str("Incapable d'ouvrir le fichier ou la page web");
        } else if lang.contains("es") {
            _return.push_str("Error al abrir el archivo o la página web");
        } else if lang.contains("eo") {
            _return.push_str("Ne eblas malfermi dosieron aŭ retpaĝon");
        } else if lang.contains("cn") {
            _return.push_str("打开文件或网页时出错");
        } else if lang.contains("tw") {
            _return.push_str("打開文件或網頁時出錯");
        } else if lang.contains("ja") {
            _return.push_str(
                "ファイルまたはWebページを開く際にエラーが発生しました",
            );
        } else if lang.contains("ko") {
            _return.push_str(
                "파일 또는 Web 페이지를 열 때 오류가 발생했습니다",
            );
        } else if lang.contains("de") {
            _return.push_str("Fehler beim Öffnen der Datei oder Webseite");
        } else {
            _return.push_str("Unable to open file or webpage");
        }
    } else if code == 10 {
        if lang.contains("fr") {
            _return.push_str("Erreur lors de l'envoi vers Imgur");
        } else if lang.contains("es") {
            _return.push_str("Error al subir a Imgur");
        } else if lang.contains("eo") {
            _return.push_str("Eraro alŝuta al Imgur");
        } else if lang.contains("cn") {
            _return.push_str("上传到Imgur时发生错误");
        } else if lang.contains("tw") {
            _return.push_str("上傳到Imgur時發生錯誤");
        } else if lang.contains("ja") {
            _return.push_str(
                "Imgurへのアップロード中にエラーが発生しました",
            );
        } else if lang.contains("ko") {
            _return.push_str(
                "Imgur에 업로드하는 중에 오류가 발생했습니다",
            );
        } else if lang.contains("de") {
            _return.push_str("Fehler beim Uploaden auf Imgur");
        } else {
            _return.push_str("Error uploading to Imgur");
        }
    } else if code == 11 {
        if lang.contains("fr") {
            _return.push_str("Erreur d'initialisation GTK");
        } else if lang.contains("es") {
            _return.push_str("Error de inicialización de GTK");
        } else if lang.contains("eo") {
            _return.push_str("GTK-komenca eraro");
        } else if lang.contains("cn") {
            _return.push_str("GTK初始化错误");
        } else if lang.contains("tw") {
            _return.push_str("GTK初始化錯誤");
        } else if lang.contains("ja") {
            _return.push_str("GTK初期化エラー");
        } else if lang.contains("ko") {
            _return.push_str("GTK 초기화 오류");
        } else if lang.contains("de") {
            _return.push_str("GTK Initialisierungsfehler");
        } else {
            _return.push_str("GTK initialize error");
        }
    } else if code == 12 {
        if lang.contains("fr") {
            _return.push_str("L'heure locale n'est pas disponible");
        } else if lang.contains("es") {
            _return.push_str("Hora local no disponible");
        } else if lang.contains("eo") {
            _return.push_str("Ne eblas akiri aktualan tempon");
        } else if lang.contains("cn") {
            _return.push_str("当地时间不可用");
        } else if lang.contains("tw") {
            _return.push_str("當地時間不可用");
        } else if lang.contains("ja") {
            _return.push_str("現地時間は利用できません");
        } else if lang.contains("ko") {
            _return.push_str("현지 시간은 사용할 수 없습니다");
        } else if lang.contains("de") {
            _return.push_str("Lokale Zeit nicht verfügbar");
        } else {
            _return.push_str("Unable to get current time");
        }
    } else if code == 13 {
        if lang.contains("fr") {
            _return.push_str("Le fichier n'est pas sauvegardé");
        } else if lang.contains("es") {
            _return.push_str("El archivo no se ha guardado");
        } else if lang.contains("eo") {
            _return.push_str("Dosiero ne savis");
        } else if lang.contains("cn") {
            _return.push_str("该文件未保存");
        } else if lang.contains("tw") {
            _return.push_str("該文件未保存");
        } else if lang.contains("ja") {
            _return.push_str("ファイルは保存されませんでした");
        } else if lang.contains("ko") {
            _return.push_str("파일이 저장되지 않았습니다");
        } else if lang.contains("de") {
            _return.push_str("Datei nicht gespeichert");
        } else {
            _return.push_str("File not saved");
        }
    } else if code == 14 {
        if lang.contains("fr") {
            _return.push_str("Commande gnome-screenshot indisponible");
        } else if lang.contains("es") {
            _return.push_str("Comando gnome-screenshot no disponible");
        } else if lang.contains("eo") {
            _return.push_str("gnome-screenshot ne ekzistas komando");
        } else if lang.contains("cn") {
            _return.push_str("「gnome-screenshot」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「gnome-screenshot」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str(
                "「gnome-screenshot」アプリケーションが利用できない",
            );
        } else if lang.contains("ko") {
            _return.push_str(
                "「gnome-screenshot」응용 프로그램을 사용할 수없는",
            );
        } else if lang.contains("de") {
            _return.push_str("gnome-screenshot command nicht verfügbar");
        } else {
            _return.push_str("gnome-screenshot command unavailable");
        }
    } else if code == 15 {
        if lang.contains("fr") {
            _return.push_str("Commande spectacle indisponible");
        } else if lang.contains("es") {
            _return.push_str("Comando spectacle no disponible");
        } else if lang.contains("eo") {
            _return.push_str("spectacle ne ekzistas komando");
        } else if lang.contains("cn") {
            _return.push_str("「spectacle」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「spectacle」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str(
                "「spectacle」アプリケーションが利用できない",
            );
        } else if lang.contains("ko") {
            _return.push_str("「spectacle」응용 프로그램을 사용할 수없는");
        } else if lang.contains("de") {
            _return.push_str("spectacle command nicht verfügbar");
        } else {
            _return.push_str("spectacle command unavailable");
        }
    } else if code == 16 {
        if lang.contains("fr") {
            _return.push_str("Commande swaygrab indisponible");
        } else if lang.contains("es") {
            _return.push_str("Comando swaygrab no disponible");
        } else if lang.contains("eo") {
            _return.push_str("swaygrab ne ekzistas komando");
        } else if lang.contains("cn") {
            _return.push_str("「swaygrab」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「swaygrab」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str(
                "「swaygrab」アプリケーションが利用できない",
            );
        } else if lang.contains("ko") {
            _return.push_str("「swaygrab」응용 프로그램을 사용할 수없는");
        } else if lang.contains("de") {
            _return.push_str("swaygrab command nicht verfügbar");
        } else {
            _return.push_str("swaygrab command unavailable");
        }
    } else if code == 17 {
        if lang.contains("fr") {
            _return.push_str("Commande maim indisponible");
        } else if lang.contains("es") {
            _return.push_str("Comando maim no disponible");
        } else if lang.contains("eo") {
            _return.push_str("maim ne ekzistas komando");
        } else if lang.contains("cn") {
            _return.push_str("「maim」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「maim」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str("「maim」アプリケーションが利用できない");
        } else if lang.contains("ko") {
            _return.push_str("「maim」응용 프로그램을 사용할 수없는");
        } else if lang.contains("de") {
            _return.push_str("maim command nicht verfügbar");
        } else {
            _return.push_str("maim command unavailable");
        }
    } else if code == 18 {
        if lang.contains("fr") {
            _return.push_str("L'environnement de bureau Wayland n'est pas pris en charge");
        } else if lang.contains("es") {
            _return.push_str("Wayland Entorno de escritorio no compatible");
        } else if lang.contains("eo") {
            _return.push_str("Wayland labortabla medio ne estas subtenata");
        } else if lang.contains("cn") {
            _return.push_str("您的Wayland桌面环境不受支持");
        } else if lang.contains("tw") {
            _return.push_str("您的Wayland桌面環境不受支持");
        } else if lang.contains("ja") {
            _return.push_str(
                "君のWaylandデスクトップ環境はサポートされていません",
            );
        } else if lang.contains("ko") {
            _return.push_str(
                "너의 Wayland 데스크탑 환경은 지원하지 않습니다",
            );
        } else if lang.contains("de") {
            _return.push_str("Wayland Desktopumgebung nicht unterstützt");
        } else {
            _return.push_str("Unsupported Wayland desktop environment");
        }
    } else if code == 19 {
        if lang.contains("fr") {
            _return.push_str("L'environnement de bureau n'est pas pris en charge");
        } else if lang.contains("es") {
            _return.push_str("Entorno de escritorio no compatible");
        } else if lang.contains("eo") {
            _return.push_str("Labortabla medio ne estas subtenata");
        } else if lang.contains("cn") {
            _return.push_str("您的桌面环境不受支持");
        } else if lang.contains("tw") {
            _return.push_str("您的桌面環境不受支持");
        } else if lang.contains("ja") {
            _return.push_str(
                "君のデスクトップ環境はサポートされていません",
            );
        } else if lang.contains("ko") {
            _return.push_str("너의 데스크탑 환경은 지원하지 않습니다");
        } else if lang.contains("de") {
            _return.push_str("Desktopumgebung nicht unterstützt");
        } else {
            _return.push_str("Unsupported desktop environment");
        }
    } else if code == 20 {
        if lang.contains("fr") {
            _return.push_str("Commande xdotool indisponible");
        } else if lang.contains("es") {
            _return.push_str("Comando xdotool no disponible");
        } else if lang.contains("eo") {
            _return.push_str("xdotool ne ekzistas komando");
        } else if lang.contains("cn") {
            _return.push_str("「xdotool」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「xdotool」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str("「xdotool」アプリケーションが利用できない");
        } else if lang.contains("ko") {
            _return.push_str("「xdotool」응용 프로그램을 사용할 수없는");
        } else if lang.contains("de") {
            _return.push_str("xdotool command nicht verfügbar");
        } else {
            _return.push_str("xdotool command unavailable");
        }
    } else if code == 21 {
        if lang.contains("fr") {
            _return.push_str("Commande feh indisponible");
        } else if lang.contains("es") {
            _return.push_str("Comando feh no disponible");
        } else if lang.contains("eo") {
            _return.push_str("feh ne ekzistas komando");
        } else if lang.contains("cn") {
            _return.push_str("「feh」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「feh」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str("「feh」アプリケーションが利用できない");
        } else if lang.contains("ko") {
            _return.push_str("「feh」응용 프로그램을 사용할 수없는");
        } else if lang.contains("de") {
            _return.push_str("feh command nicht verfügbar");
        } else {
            _return.push_str("feh command unavailable");
        }
    } else if code == 22 {
        if lang.contains("fr") {
            _return.push_str("ImageMagick indisponible");
        } else if lang.contains("es") {
            _return.push_str("ImageMagick no disponible");
        } else if lang.contains("eo") {
            _return.push_str("ImageMagick ne ekzistas komando");
        } else if lang.contains("cn") {
            _return.push_str("「ImageMagick」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「ImageMagick」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str("「ImageMagick」アプリが利用できない");
        } else if lang.contains("ko") {
            _return.push_str(
                "「ImageMagick」응용 프로그램을 사용할 수없는",
            );
        } else if lang.contains("de") {
            _return.push_str("ImageMagick nicht verfügbar");
        } else {
            _return.push_str("ImageMagick unavailable");
        }
    } else if code == 23 {
        if lang.contains("fr") {
            _return.push_str("Commande slop indisponible");
        } else if lang.contains("es") {
            _return.push_str("Comando slop no disponible");
        } else if lang.contains("eo") {
            _return.push_str("slop ne ekzistas komando");
        } else if lang.contains("cn") {
            _return.push_str("「slop」应用程序不可用");
        } else if lang.contains("tw") {
            _return.push_str("「slop」應用程序不可用");
        } else if lang.contains("ja") {
            _return.push_str("「slop」アプリケーションが利用できない");
        } else if lang.contains("ko") {
            _return.push_str("「slop」응용 프로그램을 사용할 수없는");
        } else if lang.contains("de") {
            _return.push_str("slop command nicht verfügbar");
        } else {
            _return.push_str("slop command unavailable");
        }
    } else if code == 24 {
        if lang.contains("fr") {
            _return.push_str("Fichier non lisible");
        } else if lang.contains("es") {
            _return.push_str("Archivo no legible");
        } else if lang.contains("eo") {
            _return.push_str("Ne eblas legi dosieron");
        } else if lang.contains("cn") {
            _return.push_str("我无法读取文件");
        } else if lang.contains("tw") {
            _return.push_str("我無法讀取文件");
        } else if lang.contains("ja") {
            _return.push_str("ファイルが読めない");
        } else if lang.contains("ko") {
            _return.push_str("파일을 읽을 수없는");
        } else if lang.contains("de") {
            _return.push_str("Datei nicht lesbar");
        } else {
            _return.push_str("Unable to read file");
        }
    } else if code == 25 {
        if lang.contains("fr") {
            _return.push_str("Le dossier existe déjà");
        } else if lang.contains("es") {
            _return.push_str("La carpeta ya existe");
        } else if lang.contains("eo") {
            _return.push_str("La dosierujo jam ekzistas");
        } else if lang.contains("cn") {
            _return.push_str("文件夹已存在");
        } else if lang.contains("tw") {
            _return.push_str("文件夾已存在");
        } else if lang.contains("ja") {
            _return.push_str("フォルダは既に存在します");
        } else if lang.contains("ko") {
            _return.push_str("폴더는 이미 존재합니다");
        } else if lang.contains("de") {
            _return.push_str("Ordner existiert bereits");
        } else {
            _return.push_str("Folder exists");
        }
    } else if code == 26 {
        if lang.contains("fr") {
            _return.push_str("Impossible de publier sur Mastodon");
        } else if lang.contains("es") {
            _return.push_str("Error al publicar al Mastodon");
        } else if lang.contains("eo") {
            _return.push_str("Malsukcesis eldoni al Mastodon");
        } else if lang.contains("cn") {
            _return.push_str("无法上传到Mastodon");
        } else if lang.contains("tw") {
            _return.push_str("無法上傳到Mastodon");
        } else if lang.contains("ja") {
            _return.push_str("Mastodonにアップロードできません");
        } else if lang.contains("ko") {
            _return.push_str("Mastodon에 업로드 할 수 없습니다");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim veröffentlichen auf Mastodon");
        } else {
            _return.push_str("Unable to send to Mastodon");
        }
    } else if code == 27 {
        if lang.contains("fr") {
            _return.push_str("Impossible de publier sur Twitter");
        } else if lang.contains("es") {
            _return.push_str("Error al publicar al Twitter");
        } else if lang.contains("eo") {
            _return.push_str("Malsukcesis eldoni al Twitter");
        } else if lang.contains("cn") {
            _return.push_str("无法上传到Twitter");
        } else if lang.contains("tw") {
            _return.push_str("無法上傳到Twitter");
        } else if lang.contains("ja") {
            _return.push_str("Twitterにアップロードできません");
        } else if lang.contains("ko") {
            _return.push_str("Twitter에 업로드 할 수 없습니다");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim veröffentlichen auf Twitter");
        } else {
            _return.push_str("Unable to send to Twitter");
        }
    }

    _return
}
