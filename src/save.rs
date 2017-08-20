use image::temp_dir;
use std::time::Duration;
use std::{env, fs, thread};
use notification;
use time;
use error;

pub fn file(file: String) {

    let tmp = temp_dir(0);
    let temp = tmp.to_str().unwrap().clone();

    thread::sleep(Duration::new(0, 500000000));

    let _copy = match fs::copy(file, tmp.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(30));
            notification::error(30);
            error::fatal()
        }
    };
    notification::file_saved(temp);
}

pub fn save() {

    // tmp gets temporary dir
    let tmp = temp_dir(0);
    let temp = tmp.to_str().unwrap().clone();

    // home gets the user's name
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

    // _dir creates pictures dir if not already there
    let _dir = match fs::create_dir(pictures) {
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

    // _clone copies the temp file to your home pic dir
    let _clone = match fs::copy(tmp.clone(), new_file) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(30));
            notification::error(30);
            return;
        }
    };
    notification::file_saved(temp);
}
