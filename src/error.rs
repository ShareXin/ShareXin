use language;
use std::process;
use yaml_rust::{Yaml, YamlLoader};

fn error_loader(code: &str, locator: &Yaml) -> String {
    let message = &locator[code].as_str().expect("Error not found");
    return format!("{}", message);
}

pub fn exit() -> ! {
    process::exit(1);
}

pub fn message(code: usize) -> String {
    let file = language::loader();
    let locators = YamlLoader::load_from_str(file).unwrap();
    let locator = &locators[0]["Error"];

    let error = &locator["Error"].as_str().unwrap();

    match code {
        1...31 => return error_formatter(code, error, locator),
        _ => unreachable!("Internal Logic Error"),
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
