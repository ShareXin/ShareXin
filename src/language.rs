use std::env;
use Destination;
use VERSION;
use SHAREXIN;
use open;
use std::*;
use curl::easy::Easy;
use yaml_rust::*;

static APP: &'static str = "sharexin ";

#[derive(Debug, Clone, Copy)]
pub struct Language {
    service: Destination,
    option: u32,
}

impl Language {
    pub fn new(service: Destination, option: u32) -> Language {
        Language {
            service: service,
            option: option,
        }
    }
}

pub fn locale() -> String {
    match env::var("LANG") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("{}", error(2));
            String::from("en_US.utf8").to_lowercase()
        }
    }
}

fn loader<'a>(lang: String) -> &'a str {

    if lang.contains("fr") {
        return include_str!("lang/fr.yml");
    } else if lang.contains("es") {
        return include_str!("lang/es.yml");
    } else if lang.contains("eo") {
        return include_str!("lang/eo.yml");
    } else if lang.contains("cn") {
        return include_str!("lang/cn.yml");
    } else if lang.contains("tw") {
        return include_str!("lang/tw.yml");
    } else if lang.contains("ja") {
        return include_str!("lang/ja.yml");
    } else if lang.contains("ko") {
        return include_str!("lang/ko.yml");
    } else if lang.contains("de") {
        return include_str!("lang/de.yml");
    }

    return include_str!("lang/en.yml");
}

pub fn notification(langue: Language) -> String {

    let file = loader(locale());
    let docs = YamlLoader::load_from_str(file).unwrap();
    let mut doc = &docs[0]["Notification"];

    if langue.option == 0 {
        doc = &doc["Sent"];
        if langue.service.mastodon {
            doc = &doc["Mastodon"];
        } else if langue.service.twitter {
            doc = &doc["Twitter"];
        } else if langue.service.imgur {
            doc = &doc["Imgur"];
        }
    } else if langue.option == 2 {
        doc = &doc["File"];
    } else if langue.option == 3 {
        doc = &doc["Empty"];
        if langue.service.mastodon {
            doc = &doc["Mastodon"];
        } else if langue.service.twitter {
            doc = &doc["Twitter"];
        }
    } else if langue.option == 4 {
        doc = &doc["Not_Sent"];
        if langue.service.mastodon {
            doc = &doc["Mastodon"];
        } else if langue.service.twitter {
            doc = &doc["Twitter"];
        }
    }
    return format!("{}", doc.as_str().unwrap());
}

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
                Err(_) => {
                    eprintln!("{}", error(18));
                    process::exit(1)
                }
            };
            let latest_version: usize = match str::replace(&latest_utf, ".", "").parse::<usize>() {
                Ok(ok) => ok,
                Err(_) => {
                    eprintln!("{}", error(18));
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
        Err(_) => {
            eprintln!("{}", error(16));
            process::exit(1)
        }
    };
}

fn check_update(latest_version: usize, current_version: usize, latest_utf: String) -> String {

    let file = loader(locale());
    let docs = YamlLoader::load_from_str(file).unwrap();
    let doc = &docs[0]["Update"];

    let installed = format!("{}", &doc["Installed"].as_str().unwrap());
    let latest = format!("{}", &doc["Latest"].as_str().unwrap());
    let mut update_state = String::new();

    if latest_version > current_version {
        update_state.push_str(&doc["Out"].as_str().unwrap());
    } else if latest_version < current_version {
        update_state.push_str(&doc["Too"].as_str().unwrap());
    } else if latest_version == current_version {
        update_state.push_str(&doc["Up"].as_str().unwrap());
    }

    if latest_version > current_version {
        open_update();
        return format!(
            "{}: {}\n{}: {}\n{}",
            &installed,
            VERSION,
            &latest,
            &latest_utf,
            &update_state
        );
    } else if latest_version < current_version {
        return format!(
            "{}: {}\n{}: {}\n{}",
            &installed,
            VERSION,
            &latest,
            &latest_utf,
            &update_state
        );
    } else if latest_version == current_version {
        return format!(
            "{}: {}\n{}: {}\n{}",
            &installed,
            VERSION,
            &latest,
            &latest_utf,
            &update_state
        );
    }
    return String::new();
}

fn open_update() {

    match open::that(SHAREXIN) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error(19));
            return;
        }
    };

    let lang = locale();

    let file = loader(lang);
    let docs = YamlLoader::load_from_str(file).unwrap();
    let doc = &docs[0]["Update"];
    let upgrade = format!("{}", &doc["Check"].as_str().unwrap());

    println!("{}: {}", upgrade, SHAREXIN);
}

fn error_loader<'a>(code: &'a str, doc: &Yaml) -> String {
    let error: &str = &code.to_string();
    let message = &doc[error].as_str().expect("Error not found");
    return format!("{}", message);
}

pub fn error(code: usize) -> String {

    let lang = match env::var("LANG") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("Error getting $LANG variable");
            String::from("en_US.utf8").to_lowercase()
        }
    };

    let file = loader(lang);
    let docs = YamlLoader::load_from_str(file).unwrap();
    let doc = &docs[0]["Error"];

    let error = &doc["Error"].as_str().unwrap();

    if code == 0 {
        return format!("Unknown error");
    } else if code == 1 {
        return format!("{} 1: {}", error, error_loader("1", doc));
    } else if code == 2 {
        return format!("{} 2: {}", error, error_loader("2", doc));
    } else if code == 3 {
        return format!("{} 3: {}", error, error_loader("3", doc));
    } else if code == 4 {
        return format!("{} 4: {}", error, error_loader("4", doc));
    } else if code == 5 {
        return format!("{} 5: {}", error, error_loader("5", doc));
    } else if code == 6 {
        return format!("{} 6: {}", error, error_loader("6", doc));
    } else if code == 7 {
        return format!("{} 7: {}", error, error_loader("7", doc));
    } else if code == 8 {
        return format!("{} 8: {}", error, error_loader("8", doc));
    } else if code == 9 {
        return format!("{} 9: {}", error, error_loader("9", doc));
    } else if code == 10 {
        return format!("{} 10: {}", error, error_loader("10", doc));
    } else if code == 11 {
        return format!("{} 11: {}", error, error_loader("11", doc));
    } else if code == 12 {
        return format!("{} 12: {}", error, error_loader("12", doc));
    } else if code == 13 {
        return format!("{} 13: {}", error, error_loader("13", doc));
    } else if code == 14 {
        return format!("{} 14: {}", error, error_loader("14", doc));
    } else if code == 15 {
        return format!("{} 15: {}", error, error_loader("15", doc));
    } else if code == 16 {
        return format!("{} 16: {}", error, error_loader("16", doc));
    } else if code == 17 {
        return format!("{} 17: {}", error, error_loader("17", doc));
    } else if code == 18 {
        return format!("{} 18: {}", error, error_loader("18", doc));
    } else if code == 19 {
        return format!("{} 19: {}", error, error_loader("19", doc));
    } else if code == 20 {
        return format!("{} 20: {}", error, error_loader("20", doc));
    } else if code == 21 {
        return format!("{} 21: {}", error, error_loader("21", doc));
    } else if code == 22 {
        return format!("{} 22: {}", error, error_loader("22", doc));
    } else if code == 23 {
        return format!("{} 23: {}", error, error_loader("23", doc));
    } else if code == 24 {
        return format!("{} 24: {}", error, error_loader("24", doc));
    } else if code == 25 {
        return format!("{} 25: {}", error, error_loader("25", doc));
    } else if code == 26 {
        return format!("{} 26: {}", error, error_loader("26", doc));
    } else if code == 27 {
        return format!("{} 27: {}", error, error_loader("27", doc));
    } else if code == 28 {
        return format!("{} 28: {}", error, error_loader("28", doc));
    } else if code == 29 {
        return format!("{} 29: {}", error, error_loader("29", doc));
    } else if code == 30 {
        return format!("{} 30: {}", error, error_loader("30", doc));
    }

    return format!("");
}

pub fn help() -> String {

    let usage_usage = "sharexin <options> [destination] [image options] [FILE]";
    let usage_examples = "  sharexin toot
  sharexin tweet full
  sharexin toot area
  sharexin imgur file [FILE]";

    let file = loader(locale());
    let docs = YamlLoader::load_from_str(file).unwrap();
    let doc = &docs[0]["Help"];

    // templates
    let usage = &doc["Usage"].as_str().unwrap();
    let options = &doc["Options"].as_str().unwrap();
    let help = &doc["Help"].as_str().unwrap();
    let version = &doc["Version"].as_str().unwrap();
    let upgrade = &doc["Upgrade"].as_str().unwrap();
    let image = &doc["Image"].as_str().unwrap();
    let area = &doc["Area"].as_str().unwrap();
    let window = &doc["Window"].as_str().unwrap();
    let full = &doc["Full"].as_str().unwrap();
    let file = &doc["File"].as_str().unwrap();
    let destinations = &doc["Destinations"].as_str().unwrap();
    let toot = &doc["Toot"].as_str().unwrap();
    let tweet = &doc["Tweet"].as_str().unwrap();
    let imgur = &doc["Imgur"].as_str().unwrap();
    let examples = &doc["Examples"].as_str().unwrap();

    return format!(
        "{}{}\n{}: {}\n\n{}:\n  -h, --help\t{}\n  -V, --version\t{}\n  -U, --upgrade\t{}\n
{}:\n  area\t\t{}\n  window\t{}\n  full\t\t{}\n  file\t\t{}\n
{}:\n  toot\t\t{}\n  tweet\t\t{}\n  imgur\t\t{}\n
{}:\n{}",
        APP,
        VERSION,
        usage,
        usage_usage,
        options,
        help,
        version,
        upgrade,
        image,
        area,
        window,
        full,
        file,
        destinations,
        toot,
        tweet,
        imgur,
        examples,
        usage_examples
    );

}
