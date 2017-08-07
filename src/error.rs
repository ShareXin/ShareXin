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
*/

use std::env;

pub fn message(code: usize) -> String {
    let mut _return = String::new();
    let _lang = env::var("LANG").unwrap();
    let lang = &_lang.to_lowercase();

    if code == 1 {
        _return.push_str("Error getting $LANG variable");
    } else if code == 0 {
        if lang.contains("fr") {
            _return.push_str("Error getting $HOME variable");
        } else {
            _return.push_str("Error getting $HOME variable");
        }
    } else if code == 2 {
        if lang.contains("fr") {
            _return.push_str("Github unreachable");
        } else {
            _return.push_str("Github unreachable");
        }
    } else if code == 3 {
        if lang.contains("fr") {
            _return.push_str("Imgur unreachable");
        } else {
            _return.push_str("Imgur unreachable");
        }
    } else if code == 4 {
        if lang.contains("fr") {
            _return.push_str("t command unavailable");
        } else {
            _return.push_str("t command unavailable");
        }
    } else if code == 5 {
        if lang.contains("fr") {
            _return.push_str("toot command unavailable");
        } else {
            _return.push_str("toot command unavailable");
        }
    } else if code == 6 {
        if lang.contains("fr") {
            _return.push_str("Error getting $XDG_SESSION_TYPE variable");
        } else {
            _return.push_str("Error getting $XDG_SESSION_TYPE variable");
        }
    } else if code == 7 {
        if lang.contains("fr") {
            _return.push_str("Error getting $DESKTOP_SESSION variable");
        } else {
            _return.push_str("Error getting $DESKTOP_SESSION variable");
        }
    } else if code == 8 {
        if lang.contains("fr") {
            _return.push_str("Error on parsing latest version number");
        } else {
            _return.push_str("Error on parsing latest version number");
        }
    } else if code == 9 {
        if lang.contains("fr") {
            _return.push_str("Unable to open file or webpage");
        } else {
            _return.push_str("Unable to open file or webpage");
        }
    } else if code == 10 {
        if lang.contains("fr") {
            _return.push_str("Error uploading to Imgur");
        } else {
            _return.push_str("Error uploading to Imgur");
        }
    } else if code == 11 {
        if lang.contains("fr") {
            _return.push_str("GTK initialize error");
        } else {
            _return.push_str("GTK initialize error");
        }
    } else if code == 12 {
        if lang.contains("fr") {
            _return.push_str("Unable to get current time");
        } else {
            _return.push_str("Unable to get current time");
        }
    } else if code == 13 {
        if lang.contains("fr") {
            _return.push_str("File not saved");
        } else {
            _return.push_str("File not saved");
        }
    } else if code == 14 {
        if lang.contains("fr") {
            _return.push_str("Gnome-screenshot command unavailable");
        } else {
            _return.push_str("Gnome-screenshot command unavailable");
        }
    } else if code == 15 {
        if lang.contains("fr") {
            _return.push_str("Spectacle command unavailable");
        } else {
            _return.push_str("Spectacle command unavailable");
        }
    } else if code == 16 {
        if lang.contains("fr") {
            _return.push_str("Swaygrab command unavailable");
        } else {
            _return.push_str("Swaygrab command unavailable");
        }
    } else if code == 17 {
        if lang.contains("fr") {
            _return.push_str("Maim command unavailable");
        } else {
            _return.push_str("Maim command unavailable");
        }
    } else if code == 18 {
        if lang.contains("fr") {
            _return.push_str("Unsupported Wayland desktop environment");
        } else {
            _return.push_str("Unsupported Wayland desktop environment");
        }
    } else if code == 19 {
        if lang.contains("fr") {
            _return.push_str("Unsupported desktop environment");
        } else {
            _return.push_str("Unsupported desktop environment");
        }
    } else if code == 20 {
        if lang.contains("fr") {
            _return.push_str("Xdotool command unavailable");
        } else {
            _return.push_str("Xdotool command unavailable");
        }
    } else if code == 21 {
        if lang.contains("fr") {
            _return.push_str("Feh command unavailable");
        } else {
            _return.push_str("Feh command unavailable");
        }
    } else if code == 22 {
        if lang.contains("fr") {
            _return.push_str("ImageMagick unavailable");
        } else {
            _return.push_str("ImageMagick unavailable");
        }
    } else if code == 23 {
        if lang.contains("fr") {
            _return.push_str("Slop command unavailable");
        } else {
            _return.push_str("Slop command unavailable");
        }
    } else if code == 24 {
        if lang.contains("fr") {
            _return.push_str("Unable to read file");
        } else {
            _return.push_str("Unable to read file");
        }
    }

    _return
}

/*
    Language order: fr, es, eo, cn, tw, ja, ko, de, en
*/
