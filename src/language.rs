use std::env;

// Retrieves locale settings of the user using LC_CTYPE or LANG
pub fn locale() -> String {
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
    } else {
        return include_str!("../lang/en.yml").to_string();
    }
}
