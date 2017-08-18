// args may be 0 for selection screenshots
// 1 for window screenshots, or 2 for fullscreenshots

#[cfg(target_os = "macos")]
use screenshot::get_screenshot;

use std::time::Duration;
use std::process::Command;
use std::{env, fs, process, thread};
use notification;
use time;
use error;

pub fn file(file: String) {

    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
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

#[cfg(target_os = "macos")]
fn screenshot(args: usize, temp: &str) {

    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");

    println!("{:?}", tmp);
}

#[cfg(not(target_os = "macos"))]
fn screenshot(args: usize, temp: &str, session: String, desktop: String) {

    match session.as_ref() {
        "wayland" => match desktop.as_ref() {
            "gnome" => gnome(args, temp),
            "cinnamon" => gnome(args, temp),
            "plasma" => kde(args, temp),
            "sway" => sway(args, temp),
            _ => {
                eprintln!("{}", error::message(26));
                notification::error(26);
                error::fatal()
            }
        },
        "x11" => match desktop.as_ref() {
            "gnome" => gnome(args, temp),
            "cinnamon" => gnome(args, temp),
            "plasma" => kde(args, temp),
            _ => scrot(args, temp),
        },
        _ => match desktop.as_ref() {
            "gnome" => gnome(args, temp),
            "plasma" => kde(args, temp),
            _ => scrot(args, temp),
        },
    }
}

#[cfg(not(target_os = "macos"))]
fn sway(args: usize, temp: &str) {

    if args == 0 {

        // _before_image takes a full screenshot using swaygrab
        match Command::new("swaygrab").arg(temp.clone()).output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(9));
                notification::error(9);
                error::fatal()
            }
        };

        // _feh displays it
        println!("Feh may not display properly due to tiling and Wayland.");

        let mut _feh = match Command::new("feh").args(&[temp.clone(), "-F"]).spawn() {
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

#[cfg(not(target_os = "macos"))]
fn gnome(args: usize, temp: &str) {

    if args == 0 {

        // _before_image takes a full screenshot using gnome0creenshot
        match Command::new("gnome-screenshot")
            .args(&["-f", temp.clone()])
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
        let mut _feh = match Command::new("feh").args(&[temp.clone(), "-F"]).spawn() {
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

#[cfg(not(target_os = "macos"))]
fn kde(args: usize, temp: &str) {

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

#[cfg(not(target_os = "macos"))]
fn scrot(args: usize, temp: &str) {

    if args == 0 {

        // _before_image takes a full screenshot using scrot
        match Command::new("scrot").arg(temp.clone()).output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };

        // _feh displays it
        let mut _feh = match Command::new("feh").args(&[temp.clone(), "-F"]).spawn() {
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

pub fn image(args: usize) {

    //  tmp gets the temporary directory of the system
    let mut tmp = env::temp_dir();

    // adds filename
    tmp.push("sharexin.png");

    // makes a string
    let temp = tmp.to_str().unwrap();

    // x11/wayland session info gotten here
    let session = match env::var("XDG_SESSION_TYPE") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("{}", error::message(3));
            String::from("x11").to_lowercase()
        }
    };

    // desktop environment info gotten here
    let desktop = match env::var("DESKTOP_SESSION") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("{}", error::message(4));
            notification::error(4);
            String::new()
        }
    };

    screenshot(args.clone(), temp.clone(), session.clone(), desktop.clone());
    if !tmp.exists() {
        eprintln!("{}", error::message(30));
        notification::error(30);
        error::fatal();
    }

    if args == 1 && desktop != "gnome" {
        //  adds a shadow
        match Command::new("convert")
            .arg(temp.clone())
            .args(&[
                "(",
                "+clone",
                "-background",
                "black",
                "-shadow",
                "80x3+5+5",
                ")",
                "+swap",
                "-background",
                "none",
                "-layers",
                "merge",
                "+repage",
            ])
            .arg(temp.clone())
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(13));
                notification::error(13);
                error::fatal()
            }
        };
        thread::sleep(Duration::new(0, 500000000));
    }

    save();
}

fn save() {

    // tmp gets temporary dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();

    // home gets the user's name
    let home = match env::var("HOME") {
        Ok(home) => if home.to_string().is_empty() {
            eprintln!("{}", error::message(1));
            notification::error(1);
            error::fatal()
        } else {
            home
        },
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
        Err(_) => {
            eprintln!("{}", error::message(29));
        }
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
