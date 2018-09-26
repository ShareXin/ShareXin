use error;
use image::temp_dir;
use notification;
use std::process::Command;
use std::{fs, process};

pub fn sway(args: usize, temp: &str) {
    if args == 0 {
        // makes filename for temporary temporary file
        let _tmp = temp_dir(1);
        let tmp = _tmp.to_str().unwrap().clone();

        // _before_image takes a full screenshot using swaygrab
        match Command::new("swaygrab").arg(&tmp).output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(9));
                notification::error(9);
                error::fatal()
            }
        };

        // _feh displays it
        println!("Feh may not display properly due to tiling and Wayland.");

        let mut _feh = match Command::new("feh").args(&[&tmp, "-F"]).spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(12));
                notification::error(12);
                error::fatal()
            }
        };

        // _image lets use _slop to select
        let _slop = match Command::new("slop").output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(14));
                notification::error(14);
                error::fatal()
            }
        };
        let slop = String::from_utf8_lossy(&_slop.stdout);
        let _image = match Command::new("convert")
            .args(&[temp.clone(), "-crop", &slop, temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(15));
                notification::error(15);
                error::fatal()
            }
        };

        // closes _feh, gently
        match _feh.kill() {
            Ok(ok) => ok,
            Err(_) => return,
        };

        match fs::remove_file(tmp) {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(0));
                notification::error(0);
                return;
            }
        };

        if _image.code() == Some(1) {
            process::exit(1);
        }
    } else if args == 1 {
        // _image uses swaygrab to get "focused" window and take screenshot
        let _image = match Command::new("swaygrab")
            .args(&["-f", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(9));
                notification::error(9);
                error::fatal()
            }
        };

        if _image.code() == Some(1) {
            process::exit(1);
        }
    } else if args == 2 {
        // _image uses swaygrab to take screenshot
        let _image = match Command::new("swaygrab").arg(temp.clone()).status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(9));
                notification::error(9);
                error::fatal()
            }
        };

        if _image.code() == Some(1) {
            process::exit(1);
        }
    }
}

pub fn gnome(args: usize, temp: &str) {
    if args == 0 {
        // makes filename for temporary temporary file
        let _tmp = temp_dir(1);
        let tmp = _tmp.to_str().unwrap().clone();

        // _before_image takes a full screenshot using gnome0creenshot
        match Command::new("gnome-screenshot")
            .args(&["-f", &tmp])
            .output()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(7));
                notification::error(7);
                error::fatal()
            }
        };

        // _feh displays it
        let mut _feh = match Command::new("feh").args(&[&tmp, "-F"]).spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(12));
                notification::error(12);
                error::fatal()
            }
        };

        // _image lets you select
        let _image = match Command::new("gnome-screenshot")
            .args(&["-a", "-f", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(7));
                notification::error(7);
                error::fatal()
            }
        };

        match fs::remove_file(tmp) {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(0));
                notification::error(0);
                return;
            }
        };

        // closes _feh, gently
        match _feh.kill() {
            Ok(ok) => ok,
            Err(_) => return,
        };
    } else if args == 1 {
        // _image uses gnome-screenshot to get current window and take screenshot
        let _image = match Command::new("gnome-screenshot")
            .args(&["-w", "-e", "shadow", "-f", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(7));
                notification::error(7);
                error::fatal()
            }
        };
    } else if args == 2 {
        // _image uses gnome-screenshot to take screenshot
        let _image = match Command::new("gnome-screenshot")
            .args(&["-f", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(7));
                notification::error(7);
                error::fatal()
            }
        };
    }
}

pub fn kde(args: usize, temp: &str) {
    if args == 0 {
        // _image pauses screen and lets you select
        let _image = match Command::new("spectacle")
            .args(&["-rbno", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };
    } else if args == 1 {
        // _image uses spectacle to get current window and take screenshot
        let _image = match Command::new("spectacle")
            .args(&["-abno", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };
    } else if args == 2 {
        // _image uses spectacle to take screenshot
        let _image = match Command::new("spectacle")
            .args(&["-fbno", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };
    }
}

pub fn scrot(args: usize, temp: &str) {
    if args == 0 {
        // makes filename for temporary temporary file
        let _tmp = temp_dir(1);
        let tmp = _tmp.to_str().unwrap().clone();

        // _before_image takes a full screenshot using scrot
        match Command::new("scrot").arg(&tmp).output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };

        // _feh displays it
        let mut _feh = match Command::new("feh").args(&[&tmp, "-F"]).spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(12));
                notification::error(12);
                error::fatal()
            }
        };

        // _image lets you select
        let _image = match Command::new("scrot")
            .args(&["--select", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };

        // closes _feh, gently
        match _feh.kill() {
            Ok(ok) => ok,
            Err(_) => return,
        };

        match fs::remove_file(tmp) {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(0));
                notification::error(0);
                return;
            }
        };

        if _image.code() == Some(2) {
            process::exit(1);
        }
    } else if args == 1 {
        // _image uses scrot to take window screenshot
        let _image = match Command::new("scrot")
            .args(&["--border", "--focused", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };
    } else if args == 2 {
        // _image uses scrot to take screenshot
        let _image = match Command::new("scrot").arg(temp.clone()).status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };
    }
}
