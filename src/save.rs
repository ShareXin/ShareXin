use error;
use glib::{get_user_special_dir, UserDirectory};
use image;
use std::{fs, path};
use time;

pub fn file(file: String) {
    let tmp = image::temp_dir();

    // Takes the file provided by the user and copies it to a temporary file
    match fs::copy(file, tmp.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(30));
            error::exit()
        }
    };
}

pub fn save() {
    let tmp: path::PathBuf = image::temp_dir();

    // Using XDG specifications, gets a user's Pictures folder
    // Can adjust to a user's localization of their Pictures folder
    // For example, if user's system is French, will use "Image" folder instead
    let xdg_pictures = get_user_special_dir(UserDirectory::Pictures).unwrap();
    let home = xdg_pictures.to_str().unwrap();

    // Creates "ShareXin" folder in user's Pictures folders
    let mut pictures = String::from(home);
    pictures.push_str("/ShareXin");
    match fs::create_dir(pictures) {
        Ok(ok) => ok,
        Err(_) => {}
    };

    // Creates a folder based on the year and month (similar to ShareX)
    let mut new_file = String::from(home);
    let folder_date = String::from(match time::strftime("%Y-%m", &time::now()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(25));
            error::exit()
        }
    });
    new_file.push_str("/ShareXin/");
    new_file.push_str(&folder_date);
    match fs::create_dir(new_file.clone()) {
        Ok(ok) => ok,
        Err(_) => {}
    };

    // Creates file name based on the year, month, day, hour, minute, and second, with .png
    new_file.push_str("/sharexin-");
    let time = String::from(match time::strftime("%Y-%m-%d-%H_%M_%S", &time::now()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(25));
            error::exit()
        }
    });
    new_file.push_str(&time);
    new_file.push_str(".png");

    // Copies the temporary file to the new file in the
    // year/month folder in ShareXin in user's Pictures folder
    match fs::copy(tmp.clone(), new_file) {
        Ok(_) => return,
        Err(_) => eprintln!("{}", error::message(30)),
    };
}
