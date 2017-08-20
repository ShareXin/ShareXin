use image::temp_dir;
use std::time::Duration;
use std::{env, fs, thread};
use notification;
use time;
use error;

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

    let home = match env::var("HOME") {
        Ok(home) => {
            if home.to_string().is_empty() {
                eprintln!("{}", error::message(1));
                notification::error(1);
                error::fatal()
            } else {
                home
            }
        }
        Err(_) => {
            eprintln!("{}", error::message(1));
            notification::error(1);
            error::fatal()
        }
    };
    let mut pictures = String::from(home.clone());
    pictures.push_str("/Pictures/ShareXin");

    match fs::create_dir(pictures) {
        Ok(ok) => ok,
        Err(_) => {}
    };
    let mut new_file = String::from(home);
    new_file.push_str("/Pictures/ShareXin/sharexin-");

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