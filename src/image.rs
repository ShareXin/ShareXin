use error;
use notification;
use open;
use save;
use screenshot_rs;
use screenshot_rs::ScreenshotKind;
use std::path::PathBuf;
use std::{env, fs};

pub fn image(kind: ScreenshotKind) {
    // tmp gets the temporary directory of the system
    let tmp = temp_dir();

    // makes a string
    let temp = tmp.to_str().unwrap().to_string();

    match kind {
        ScreenshotKind::Area => screenshot_rs::screenshot_area(temp),
        ScreenshotKind::Window => screenshot_rs::screenshot_window(temp),
        ScreenshotKind::Full => screenshot_rs::screenshot_full(temp),
    }

    if !tmp.is_file() {
        eprintln!("{}", error::message(30));
        error::exit();
    }

    save::save();
}

pub fn delete_temp() {
    match fs::remove_file(temp_dir()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(0));
        }
    }
}

pub fn open_temp() {
    match open::that(temp_dir()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(19));
            notification::error(19);
            return;
        }
    };
}

pub fn temp_dir() -> PathBuf {
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    return tmp;
}
