use language;
use std::process;
use yaml_rust::YamlLoader;

// Ends the current process with an error code (useful in BASH scripting)
pub fn exit() -> ! {
    process::exit(1);
}

// Gets error message from appropriate localization file provided by language::loader()
// and returns it as a String
pub fn message(code: usize) -> String {
    let locators = YamlLoader::load_from_str(&language::loader()).unwrap();
    let locator = &locators[0]["Error"];

    let error = &locator["Error"].as_str().unwrap();

    match code {
        1...31 => {
            return format!(
                "{} {}: {}",
                error,
                code,
                &locator[code].as_str().expect("Error not found")
            )
        }
        _ => unreachable!("Internal Logic Error"),
    };
}
