use error;
use notification;
use open;
use save;
use screenshot_rs;
use screenshot_rs::ScreenshotKind;
use std::path::PathBuf;
use std::{env, fs};

pub fn image(kind: ScreenshotKind) {
    let tmp = temp_dir();
    let temp = tmp.to_str().unwrap().to_string();

    // Matches the kind of screenshot to functions in my screenshot-rs library
    match kind {
        ScreenshotKind::Area => screenshot_rs::screenshot_area(temp),
        ScreenshotKind::Window => screenshot_rs::screenshot_window(temp),
        ScreenshotKind::Full => screenshot_rs::screenshot_full(temp),
    }

    // If the temporary file sent to screenshot_rs doesn't exist (means the screenshot wasn't made), then exit
    if !tmp.is_file() {
        eprintln!("{}", error::message(30));
        error::exit();
    }

    // Saves the temporary file to user's Picture folder
    save::save();
}

// After a status is sent, or a status is canceled,
// or a status is unable to be sent, the temporary file will be deleted
pub fn delete_temp() {
    if fs::remove_file(temp_dir()).is_err() {
        eprintln!("{}", error::message(0));
    }
}

// Opens the temporary file in the default image viewer
pub fn open_temp() {
    if open::that(temp_dir()).is_err() {
        eprintln!("{}", error::message(19));
        notification::error(19);
        return;
    };
}

// Retrieves a system's default temporary file directory
// and appends the file name and extension to it
pub fn temp_dir() -> PathBuf {
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    return tmp;
}
