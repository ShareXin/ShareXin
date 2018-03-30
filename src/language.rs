use std::env;
use Destination;
use notification;
use error;

#[derive(Debug, Clone, Copy)]
pub struct Language {
    pub service: Destination,
    pub option: usize,
}

impl Language {
    pub fn new(service: Destination, option: usize) -> Language {
        Language {
            service: service,
            option: option,
        }
    }
}

pub fn locale() -> String {

    match env::var("LANG") {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(2));
            notification::error(2);
            String::from("en_US.utf8")
        }
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
