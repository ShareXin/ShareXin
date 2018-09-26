use language;
use std::time::Duration;
use std::{process, thread};
use yaml_rust::{Yaml, YamlLoader};

fn error_loader<'a>(code: &'a str, locator: &Yaml) -> String {
    let error: &str = &code.to_string();
    let message = &locator[error].as_str().expect("Error not found");
    return format!("{}", message);
}

pub fn fatal() -> ! {
    let file = language::loader();
    let locators = YamlLoader::load_from_str(file).unwrap();
    let locator = &locators[0]["Error"];

    println!("{}", error_loader("Fatal", locator));
    thread::sleep(Duration::new(1, 5));
    process::exit(1);
}

pub fn message(code: usize) -> String {
    let file = language::loader();
    let locators = YamlLoader::load_from_str(file).unwrap();
    let locator = &locators[0]["Error"];

    let error = &locator["Error"].as_str().unwrap();

    match code {
        1...31 => return error_formatter(code, error, locator),
        _ => return String::from("Unknown error"),
    };
}

fn error_formatter(code: usize, error: &str, locator: &Yaml) -> String {
    return format!(
        "{} {}: {}",
        error,
        code,
        error_loader(&code.to_string(), locator)
    );
}
