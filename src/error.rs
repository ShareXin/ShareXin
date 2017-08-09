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

use std::env;

pub fn message(code: usize) -> String {
    let mut _return = String::new();
    let _lang = match env::var("LANG") {
        Ok(ok) => ok,
        Err(_) => String::from("en_US.utf8"),
    };
    let lang = &_lang.to_lowercase();

    if code == 1 {
        _return.push_str("Error getting $LANG variable");
    } else if code == 0 {
        if lang.contains("fr") {
            _return.push_str("Error getting $HOME variable");
        } else if lang.contains("es") {
            _return.push_str("Error al obtener la variable $HOME");
        } else if lang.contains("eo") {
            _return.push_str("Eraro ricevanta la $HOME variablon");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim lesen der $HOME Variable");
        } else {
            _return.push_str("Error getting $HOME variable");
        }
    } else if code == 2 {
        if lang.contains("fr") {
            _return.push_str("Github unreachable");
        } else if lang.contains("es") {
            _return.push_str("Github inalcanzable");
        } else if lang.contains("de") {
            _return.push_str("Github unerreichbar");
        } else {
            _return.push_str("Github unreachable");
        }
    } else if code == 3 {
        if lang.contains("fr") {
            _return.push_str("Imgur unreachable");
        } else if lang.contains("es") {
            _return.push_str("Imgur inalcanzable");
        } else if lang.contains("de") {
            _return.push_str("Imgur unerreichbar");
        } else {
            _return.push_str("Imgur unreachable");
        }
    } else if code == 4 {
        if lang.contains("fr") {
            _return.push_str("t command unavailable");
        } else if lang.contains("es") {
            _return.push_str("t comando no disponible");
        } else if lang.contains("de") {
            _return.push_str("t command nicht verfügbar");
        } else {
            _return.push_str("t command unavailable");
        }
    } else if code == 5 {
        if lang.contains("fr") {
            _return.push_str("toot command unavailable");
        } else if lang.contains("es") {
            _return.push_str("toot comando no disponible");
        } else if lang.contains("de") {
            _return.push_str("toot command nicht verfügbar");
        } else {
            _return.push_str("toot command unavailable");
        }
    } else if code == 6 {
        if lang.contains("fr") {
            _return.push_str("Error getting $XDG_SESSION_TYPE variable");
        } else if lang.contains("es") {
            _return.push_str("Error al obtener la variable $XDG_SESSION_TYPE");
        } else if lang.contains("eo") {
            _return.push_str("Eraro ricevanta la $XDG_SESSION_TYPE variablon");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim lesen der $XDG_SESSION_TYPE Variable");
        } else {
            _return.push_str("Error getting $XDG_SESSION_TYPE variable");
        }
    } else if code == 7 {
        if lang.contains("fr") {
            _return.push_str("Error getting $DESKTOP_SESSION variable");
        } else if lang.contains("es") {
            _return.push_str("Error al obtener la variable $DESKTOP_SESSION");
        } else if lang.contains("eo") {
            _return.push_str("Eraro ricevanta la $DESKTOP_SESSION variablon");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim lesen der $DESKTOP_SESSION Variable");
        } else {
            _return.push_str("Error getting $DESKTOP_SESSION variable");
        }
    } else if code == 8 {
        if lang.contains("fr") {
            _return.push_str("Error on parsing latest version number");
        } else if lang.contains("es") {
            _return.push_str("Error al analizar el último número de versión");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim Parsen der neuesten Versionsnummer");
        } else {
            _return.push_str("Error on parsing latest version number");
        }
    } else if code == 9 {
        if lang.contains("fr") {
            _return.push_str("Unable to open file or webpage");
        } else if lang.contains("es") {
            _return.push_str("Error al abrir el archivo o la página web");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim Öffnen der Datei oder Webseite");
        } else {
            _return.push_str("Unable to open file or webpage");
        }
    } else if code == 10 {
        if lang.contains("fr") {
            _return.push_str("Error uploading to Imgur");
        } else if lang.contains("es") {
            _return.push_str("Error al subir a Imgur");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim Uploaden auf Imgur");
        } else {
            _return.push_str("Error uploading to Imgur");
        }
    } else if code == 11 {
        if lang.contains("fr") {
            _return.push_str("GTK initialize error");
        } else if lang.contains("es") {
            _return.push_str("Error de inicialización de GTK");
        } else if lang.contains("de") {
            _return.push_str("GTK Initialisierungsfehler");
        } else {
            _return.push_str("GTK initialize error");
        }
    } else if code == 12 {
        if lang.contains("fr") {
            _return.push_str("Unable to get current time");
        } else if lang.contains("es") {
            _return.push_str("Hora local no disponible");
        } else if lang.contains("de") {
            _return.push_str("Lokale Zeit nicht verfügbar");
        } else {
            _return.push_str("Unable to get current time");
        }
    } else if code == 13 {
        if lang.contains("fr") {
            _return.push_str("File not saved");
        } else if lang.contains("es") {
            _return.push_str("El archivo no se ha guardado");
        } else if lang.contains("de") {
            _return.push_str("Datei nicht gespeichert");
        } else {
            _return.push_str("File not saved");
        }
    } else if code == 14 {
        if lang.contains("fr") {
            _return.push_str("Gnome-screenshot command unavailable");
        } else if lang.contains("es") {
            _return.push_str("Comando Gnome-screenshot no disponible");
        } else if lang.contains("de") {
            _return.push_str("Gnome-screenshot command nicht verfügbar");
        } else {
            _return.push_str("Gnome-screenshot command unavailable");
        }
    } else if code == 15 {
        if lang.contains("fr") {
            _return.push_str("Spectacle command unavailable");
        } else if lang.contains("es") {
            _return.push_str("Comando Spectacle no disponible");
        } else if lang.contains("de") {
            _return.push_str("Spectacle command nicht verfügbar");
        } else {
            _return.push_str("Spectacle command unavailable");
        }
    } else if code == 16 {
        if lang.contains("fr") {
            _return.push_str("Swaygrab command unavailable");
        } else if lang.contains("es") {
            _return.push_str("Comando Swaygrab no disponible");
        } else if lang.contains("de") {
            _return.push_str("Swaygrab command nicht verfügbar");
        } else {
            _return.push_str("Swaygrab command unavailable");
        }
    } else if code == 17 {
        if lang.contains("fr") {
            _return.push_str("Maim command unavailable");
        } else if lang.contains("es") {
            _return.push_str("Comando Maim no disponible");
        } else if lang.contains("de") {
            _return.push_str("Maim command nicht verfügbar");
        } else {
            _return.push_str("Maim command unavailable");
        }
    } else if code == 18 {
        if lang.contains("fr") {
            _return.push_str("Unsupported Wayland desktop environment");
        } else if lang.contains("es") {
            _return.push_str("Wayland Entorno de escritorio no compatible");
        } else if lang.contains("de") {
            _return.push_str("Wayland Desktopumgebung nicht unterstützt");
        } else {
            _return.push_str("Unsupported Wayland desktop environment");
        }
    } else if code == 19 {
        if lang.contains("fr") {
            _return.push_str("Unsupported desktop environment");
        } else if lang.contains("es") {
            _return.push_str("Entorno de escritorio no compatible");
        } else if lang.contains("de") {
            _return.push_str("Desktopumgebung nicht unterstützt");
        } else {
            _return.push_str("Unsupported desktop environment");
        }
    } else if code == 20 {
        if lang.contains("fr") {
            _return.push_str("Xdotool command unavailable");
        } else if lang.contains("es") {
            _return.push_str("Comando Xdotool no disponible");
        } else if lang.contains("de") {
            _return.push_str("Xdotool command nicht verfügbar");
        } else {
            _return.push_str("Xdotool command unavailable");
        }
    } else if code == 21 {
        if lang.contains("fr") {
            _return.push_str("Feh command unavailable");
        } else if lang.contains("es") {
            _return.push_str("Comando Feh no disponible");
        } else if lang.contains("de") {
            _return.push_str("Feh command nicht verfügbar");
        } else {
            _return.push_str("Feh command unavailable");
        }
    } else if code == 22 {
        if lang.contains("fr") {
            _return.push_str("ImageMagick unavailable");
        } else if lang.contains("es") {
            _return.push_str("ImageMagick no disponible");
        } else if lang.contains("de") {
            _return.push_str("ImageMagick nicht verfügbar");
        } else {
            _return.push_str("ImageMagick unavailable");
        }
    } else if code == 23 {
        if lang.contains("fr") {
            _return.push_str("Slop command unavailable");
        } else if lang.contains("es") {
            _return.push_str("Comando Slop no disponible");
        } else if lang.contains("de") {
            _return.push_str("Slop command nicht verfügbar");
        } else {
            _return.push_str("Slop command unavailable");
        }
    } else if code == 24 {
        if lang.contains("fr") {
            _return.push_str("Unable to read file");
        } else if lang.contains("es") {
            _return.push_str("Archivo no legible");
        } else if lang.contains("de") {
            _return.push_str("Datei nicht lesbar");
        } else {
            _return.push_str("Unable to read file");
        }
    } else if code == 25 {
        if lang.contains("fr") {
            _return.push_str("Folder exists");
        } else if lang.contains("es") {
            _return.push_str("La carpeta ya existe");
        } else if lang.contains("de") {
            _return.push_str("Ordner existiert bereits");
        } else {
            _return.push_str("Folder exists");
        }
    } else if code == 26 {
        if lang.contains("fr") {
            _return.push_str("Unable to send to Mastodon");
        } else if lang.contains("es") {
            _return.push_str("Error al publicar al Mastodon");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim veröffentlichen auf Mastodon");
        } else {
            _return.push_str("Unable to send to Mastodon");
        }
    } else if code == 27 {
        if lang.contains("fr") {
            _return.push_str("Unable to send to Twitter");
        } else if lang.contains("es") {
            _return.push_str("Error al publicar al Twitter");
        } else if lang.contains("de") {
            _return.push_str("Fehler beim veröffentlichen auf Twitter");
        } else {
            _return.push_str("Unable to send to Twitter");
        }
    }

    _return
}

/*
    Language order: fr, es, eo, cn, tw, ja, ko, de, en
*/
