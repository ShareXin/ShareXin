use error;
use glib::{get_user_special_dir, UserDirectory};
use image::temp_dir;
use notification;
use std::time::Duration;
use std::{fs, thread};
use time;

pub fn file(file: String) {
    let tmp = temp_dir(0);

    match fs::copy(file, tmp.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(30));
            error::fatal()
        }
    };
}

pub fn save() {
    let tmp = temp_dir(0);
    let temp = tmp.to_str().unwrap().clone();

    let xdg_pictures = get_user_special_dir(UserDirectory::Pictures).unwrap();
    let home = xdg_pictures.to_str().unwrap();

    let mut pictures = String::from(home);
    pictures.push_str("/ShareXin");

    match fs::create_dir(pictures) {
        Ok(ok) => ok,
        Err(_) => {}
    };

    let mut new_file = String::from(home);
    let folder_date = String::from(match time::strftime("%Y-%m", &time::now()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(25));
            notification::error(25);
            error::fatal()
        }
    });

    new_file.push_str("/ShareXin/");
    new_file.push_str(&folder_date);
    new_file.push_str("/sharexin-");

    // time gets the time in a nice format
    let time = String::from(match time::strftime("%Y-%m-%d-%H_%M_%S", &time::now()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(25));
            notification::error(25);
            error::fatal()
        }
    });
    new_file.push_str(&time);
    new_file.push_str(".png");

    thread::sleep(Duration::new(0, 500000000));

    match fs::copy(tmp.clone(), new_file) {
        Ok(ok) => ok,
        Err(_) => 0,
    };
    notification::file_saved(temp);
}
