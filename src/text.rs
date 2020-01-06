use std::{env, process};
use yaml_rust::YamlLoader;

#[allow(dead_code)]
pub enum Errors {
    GtkInitError,
    ScreenshotUnsuccessful,
    StrfTimeUnavailable,
    PicturesFolderUnavailable,
    ScreenshotSaveError,
    ImageViewerError,
    FileError,
    ImgurUploadFailure,
    ClipboardUnavailable,
    MastodonImageUnsuccessful,
    MastodonTootUnsuccessful,
    MastodonLoginError,
    TwitterImageUnsuccessful,
    TwitterTweetUnsucessful,
    TwitterLoginError,
    NotificationUnavailable,
}

#[allow(dead_code)]
pub enum Text {
    UnsupportedWayland,
}

// Retrieves locale settings of the user using LC_CTYPE or LANG
fn locale() -> String {
    match env::var("LC_CTYPE") {
        Ok(ok) => ok,
        Err(_) => match env::var("LANG") {
            Ok(ok) => ok,
            Err(_) => String::new(),
        },
    }
}

// Retrieves the correct localization file and returns a String
pub fn loader() -> String {
    let lang = locale();

    if lang.contains("fr") {
        return include_str!("../lang/fr.yml").to_string();
    } else if lang.contains("es") {
        return include_str!("../lang/es.yml").to_string();
    } else if lang.contains("eo") {
        return include_str!("../lang/eo.yml").to_string();
    } else if lang.contains("CN") && lang.contains("zh") {
        return include_str!("../lang/cn.yml").to_string();
    } else if lang.contains("TW") && lang.contains("zh") {
        return include_str!("../lang/tw.yml").to_string();
    } else if lang.contains("ja") {
        return include_str!("../lang/ja.yml").to_string();
    } else if lang.contains("ko") {
        return include_str!("../lang/ko.yml").to_string();
    } else if lang.contains("de") {
        return include_str!("../lang/de.yml").to_string();
    } else if lang.contains("pl") {
        return include_str!("../lang/pl.yml").to_string();
    } else if lang.contains("pt") {
        return include_str!("../lang/pt.yml").to_string();
    } else if lang.contains("sv") {
        return include_str!("../lang/sv.yml").to_string();
    } else {
        return include_str!("../lang/en.yml").to_string();
    }
}

// Ends the current process with an error code (useful in BASH scripting)
pub fn exit() -> ! {
    process::exit(1);
}

// Gets error message from appropriate localization file provided by language::loader()
// and returns it as a String
pub fn message(code: usize) -> String {
    let locators = YamlLoader::load_from_str(&loader()).unwrap();
    let locator = &locators[0]["Error"];

    let error = &locator["Error"].as_str().unwrap();
    match code {
        1..=31 => return format!("{} {}: {}", error, code, &locator[code].as_str().unwrap()),
        _ => unreachable!("Internal Logic Error"),
    };
}
