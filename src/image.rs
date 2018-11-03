use glib::{get_user_special_dir, UserDirectory};
use notification;
use open;
use screenshot_rs;
use screenshot_rs::ScreenshotKind;
use std::path::PathBuf;
use std::{env, fs, path};
use text;
use time;

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
        eprintln!("{}", text::message(30));
        text::exit();
    }

    // Using XDG specifications, gets a user's Pictures folder
    // Can adjust to a user's localization of their Pictures folder
    // For example, if user's system is Spanish, will use "Fotos" folder instead
    // Then creates a folder named "ShareXin" inside that folder
    let xdg_pictures = get_user_special_dir(UserDirectory::Pictures).unwrap();
    let home = xdg_pictures.to_str().unwrap();
    let mut file = String::from(home);
    file.push_str("/ShareXin/");

    // Creates a folder name based on the year and month (similar to ShareX)
    let folder_date = String::from(match time::strftime("%Y-%m", &time::now()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", text::message(25));
            text::exit()
        }
    });
    file.push_str(&folder_date);

    // Creates a ShareXin folder and a folder named after the year-month
    if fs::create_dir_all(file.clone()).is_err() {
        if !path::Path::new(&file).is_dir() {
            eprintln!("{}", text::message(30));
        }
    };

    // Creates file name based on the year, month, day, hour, minute, and second, with .png
    file.push_str("/sharexin-");
    let time = String::from(match time::strftime("%Y-%m-%d-%H_%M_%S", &time::now()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", text::message(25));
            text::exit()
        }
    });
    file.push_str(&time);
    file.push_str(".png");

    // Copies the temporary file to the new file in the
    // year/month folder in ShareXin in user's Pictures folder
    if fs::copy(tmp.clone(), file).is_err() {
        eprintln!("{}", text::message(30));
    };
}

pub fn file(file: String) {
    let tmp = temp_dir();

    // Takes the file provided by the user and copies it to a temporary file
    if fs::copy(file, tmp.clone()).is_err() {
        eprintln!("{}", text::message(30));
        text::exit();
    };
}

// After a status is sent, or a status is canceled,
// or a status is unable to be sent, the temporary file will be deleted
pub fn delete_temp() {
    fs::remove_file(temp_dir()).unwrap();
}

// Opens the temporary file in the default image viewer
pub fn open_temp() {
    if open::that(temp_dir()).is_err() {
        eprintln!("{}", text::message(19));
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
