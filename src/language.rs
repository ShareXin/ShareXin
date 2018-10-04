use std::env;

pub fn locale() -> String {
    match env::var("LC_CTYPE") {
        Ok(ok) => ok,
        Err(_) => match env::var("LANG") {
            Ok(ok) => ok,
            Err(_) => String::new(),
        },
    }
}

pub fn loader<'a>() -> &'a str {
    let lang = locale();

    if lang.contains("fr") {
        return include_str!("lang/fr.yml");
    } else if lang.contains("es") {
        return include_str!("lang/es.yml");
    } else if lang.contains("eo") {
        return include_str!("lang/eo.yml");
    } else if lang.contains("CN") && lang.contains("zh") {
        return include_str!("lang/cn.yml");
    } else if lang.contains("TW") && lang.contains("zh") {
        return include_str!("lang/tw.yml");
    } else if lang.contains("ja") {
        return include_str!("lang/ja.yml");
    } else if lang.contains("ko") {
        return include_str!("lang/ko.yml");
    } else if lang.contains("de") {
        return include_str!("lang/de.yml");
    } else if lang.contains("pl") {
        return include_str!("lang/pl.yml");
    } else if lang.contains("pt") {
        return include_str!("lang/pt.yml");
    } else {
        return include_str!("lang/en.yml");
    }
}
