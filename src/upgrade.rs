use curl::easy::Easy;
use error;
use language;
use yaml_rust::YamlLoader;

static VERSION: &'static str = crate_version!();
static SHAREXIN: &'static str = "https://crates.io/crates/sharexin";

pub fn upgrade() {
    let mut dst = Vec::new();
    let mut latest = Easy::new();

    // Downloads the "version" file from GitHub
    latest
        .url("https://raw.githubusercontent.com/ShareXin/ShareXin/master/version")
        .unwrap();
    let mut transfer = latest.transfer();
    transfer
        .write_function(|data| {
            dst.extend_from_slice(data);
            // Removes periods from version to compare
            let mut latest_utf = String::from_utf8(dst.clone()).unwrap();
            while latest_utf.ends_with("\n") {
                let len = latest_utf.len();
                let new_len = len.saturating_sub("\n".len());
                latest_utf.truncate(new_len);
            }
            let current_version: usize = str::replace(VERSION, ".", "").parse::<usize>().unwrap();
            let latest_version: usize = match str::replace(&latest_utf, ".", "").parse::<usize>() {
                Ok(ok) => ok,
                Err(_) => {
                    eprintln!("{}", error::message(18));
                    error::exit()
                }
            };
            check_update(latest_version, current_version, latest_utf);
            Ok(data.len())
        }).unwrap();

    match transfer.perform() {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("{}, {:?}", error::message(16), e);
            error::exit()
        }
    };
}

fn check_update(latest_version: usize, current_version: usize, latest_utf: String) {
    let locators = YamlLoader::load_from_str(&language::loader()).unwrap();
    let locator = &locators[0]["Update"];

    let installed = format!("{}", &locator["Installed"].as_str().unwrap());
    let latest = format!("{}", &locator["Latest"].as_str().unwrap());
    let mut update_state = String::new();

    // Checks version numbers and decides if you are up-to-date or not
    if latest_version > current_version {
        update_state.push_str(&locator["Out"].as_str().unwrap());
    } else if latest_version < current_version {
        update_state.push_str(&locator["Too"].as_str().unwrap());
    } else if latest_version == current_version {
        update_state.push_str(&locator["Up"].as_str().unwrap());
    }
    if latest_version > current_version {
        println!(
            "{}: {}\n{}: {}\n{}",
            &installed, VERSION, &latest, &latest_utf, &update_state
        );
        let upgrade = format!("{}", &locator["Check"].as_str().unwrap());
        println!("{}: {}", upgrade, SHAREXIN);
    } else if latest_version < current_version {
        println!(
            "{}: {}\n{}: {}\n{}",
            &installed, VERSION, &latest, &latest_utf, &update_state
        );
    } else if latest_version == current_version {
        println!(
            "{}: {}\n{}: {}\n{}",
            &installed, VERSION, &latest, &latest_utf, &update_state
        );
    }
}
