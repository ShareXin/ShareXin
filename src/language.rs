use std::env;
use VERSION;
use Destination;
use yaml_rust::YamlLoader;
use error;

static APP: &'static str = "sharexin ";

#[derive(Debug, Clone, Copy)]
pub struct Language {
    pub service: Destination,
    pub option: u32,
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
            eprintln!("{}", error::message(2));
            String::from("en_US.utf8").to_lowercase()
        }
    }
}

pub fn loader<'a>(lang: String) -> &'a str {

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

pub fn help() -> String {

    let usage_usage = "sharexin <options> [destination] [image options] [FILE]";
    let usage_examples = "  sharexin toot
  sharexin tweet full
  sharexin toot area
  sharexin imgur file [FILE]";

    let file = loader(locale());
    let locators = YamlLoader::load_from_str(file).unwrap();
    let locator = &locators[0]["Help"];

    // templates
    let usage = &locator["Usage"].as_str().unwrap();
    let options = &locator["Options"].as_str().unwrap();
    let help = &locator["Help"].as_str().unwrap();
    let version = &locator["Version"].as_str().unwrap();
    let upgrade = &locator["Upgrade"].as_str().unwrap();
    let image = &locator["Image"].as_str().unwrap();
    let area = &locator["Area"].as_str().unwrap();
    let window = &locator["Window"].as_str().unwrap();
    let full = &locator["Full"].as_str().unwrap();
    let file = &locator["File"].as_str().unwrap();
    let destinations = &locator["Destinations"].as_str().unwrap();
    let toot = &locator["Toot"].as_str().unwrap();
    let tweet = &locator["Tweet"].as_str().unwrap();
    let imgur = &locator["Imgur"].as_str().unwrap();
    let examples = &locator["Examples"].as_str().unwrap();

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
