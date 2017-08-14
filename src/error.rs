use language;
use std::{env, process, thread};
use yaml_rust::{Yaml, YamlLoader};
use std::time::Duration;

fn error_loader<'a>(code: &'a str, locator: &Yaml) -> String {

    let error: &str = &code.to_string();
    let message = &locator[error].as_str().expect("Error not found");
    return format!("{}", message);

}

pub fn fatal() -> ! {

    let lang = match env::var("LANG") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("Error getting $LANG variable");
            String::from("en_US.utf8").to_lowercase()
        }
    };

    let file = language::loader(lang);
    let locators = YamlLoader::load_from_str(file).unwrap();
    let locator = &locators[0]["Error"];

    println!("{}", error_loader("Fatal", locator));
    thread::sleep(Duration::new(1, 5));
    process::exit(1);
}

pub fn message(code: usize) -> String {

    let lang = match env::var("LANG") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("Error getting $LANG variable");
            String::from("en_US.utf8").to_lowercase()
        }
    };

    let file = language::loader(lang);
    let locators = YamlLoader::load_from_str(file).unwrap();
    let locator = &locators[0]["Error"];

    let error = &locator["Error"].as_str().unwrap();

    if code == 1 {
        return format!("{} 1: {}", error, error_loader("1", locator));
    } else if code == 2 {
        return format!("{} 2: {}", error, error_loader("2", locator));
    } else if code == 3 {
        return format!("{} 3: {}", error, error_loader("3", locator));
    } else if code == 4 {
        return format!("{} 4: {}", error, error_loader("4", locator));
    } else if code == 5 {
        return format!("{} 5: {}", error, error_loader("5", locator));
    } else if code == 6 {
        return format!("{} 6: {}", error, error_loader("6", locator));
    } else if code == 7 {
        return format!("{} 7: {}", error, error_loader("7", locator));
    } else if code == 8 {
        return format!("{} 8: {}", error, error_loader("8", locator));
    } else if code == 9 {
        return format!("{} 9: {}", error, error_loader("9", locator));
    } else if code == 10 {
        return format!("{} 10: {}", error, error_loader("10", locator));
    } else if code == 11 {
        return format!("{} 11: {}", error, error_loader("11", locator));
    } else if code == 12 {
        return format!("{} 12: {}", error, error_loader("12", locator));
    } else if code == 13 {
        return format!("{} 13: {}", error, error_loader("13", locator));
    } else if code == 14 {
        return format!("{} 14: {}", error, error_loader("14", locator));
    } else if code == 15 {
        return format!("{} 15: {}", error, error_loader("15", locator));
    } else if code == 16 {
        return format!("{} 16: {}", error, error_loader("16", locator));
    } else if code == 17 {
        return format!("{} 17: {}", error, error_loader("17", locator));
    } else if code == 18 {
        return format!("{} 18: {}", error, error_loader("18", locator));
    } else if code == 19 {
        return format!("{} 19: {}", error, error_loader("19", locator));
    } else if code == 20 {
        return format!("{} 20: {}", error, error_loader("20", locator));
    } else if code == 21 {
        return format!("{} 21: {}", error, error_loader("21", locator));
    } else if code == 22 {
        return format!("{} 22: {}", error, error_loader("22", locator));
    } else if code == 23 {
        return format!("{} 23: {}", error, error_loader("23", locator));
    } else if code == 24 {
        return format!("{} 24: {}", error, error_loader("24", locator));
    } else if code == 25 {
        return format!("{} 25: {}", error, error_loader("25", locator));
    } else if code == 26 {
        return format!("{} 26: {}", error, error_loader("26", locator));
    } else if code == 27 {
        return format!("{} 27: {}", error, error_loader("27", locator));
    } else if code == 28 {
        return format!("{} 28: {}", error, error_loader("28", locator));
    } else if code == 29 {
        return format!("{} 29: {}", error, error_loader("29", locator));
    } else if code == 30 {
        return format!("{} 30: {}", error, error_loader("30", locator));
    } else {
        return format!("Unknown error");
    }
}
