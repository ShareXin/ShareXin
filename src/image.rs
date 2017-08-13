// args may be 0 for selection screenshots
// 1 for window screenshots, or 2 for fullscreenshots

#[cfg(target_os = "macos")]
use screenshot::get_screenshot;

use std::time::Duration;
use std::process::*;
use std::*;
use std;
use notification;
use time;
use language;

pub fn file(file: String) {

    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");

    thread::sleep(Duration::new(0, 500000000));

    let _copy = match std::fs::copy(file, tmp.clone()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(30));
            process::exit(1)
        }
    };
    notification::file_saved();
}

#[cfg(target_os = "macos")]
fn screenshot(args: usize, temp: &str) {

    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");

    let screenshot = get_screenshot(0).unwrap();

    image::save_buffer(
        &Path::new(tmp),
        s.as_slice(),
        s.width() as u32,
        s.height() as u32,
        image::RGBA(8),
    ).unwrap();
}

#[cfg(not(target_os = "macos"))]
fn screenshot(args: usize, temp: &str, session: String, desktop: String) {

    match session.as_ref() {
        "wayland" => match desktop.as_ref() {
            "gnome" => gnome(args, temp),
            "plasma" => kde(args, temp),
            "sway" => sway(args, temp),
            _ => {
                eprintln!("{}", language::error(26));
                process::exit(1)
            }
        },
        "x11" => match desktop.as_ref() {
            "gnome" => gnome(args, temp),
            "plasma" => kde(args, temp),
            _ => maim(args, temp),
        },
        _ => match desktop.as_ref() {
            "gnome" => gnome(args, temp),
            "plasma" => kde(args, temp),
            _ => maim(args, temp),
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
                eprintln!("{}", language::error(9));
                process::exit(1)
            }
        };

        // _feh displays it
        println!("Feh may not display properly due to tiling and Wayland.");

        match Command::new("feh").args(&[temp.clone(), "-F"]).spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(12));
                process::exit(1)
            }
        };

        // _image lets use _slop to select
        let _slop = match Command::new("slop").output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(14));
                process::exit(1)
            }
        };
        let slop = String::from_utf8_lossy(&_slop.stdout);
        let _image = match Command::new("convert")
            .args(&[temp.clone(), "-crop", &slop, temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(15));
                process::exit(1)
            }
        };

        // _kill closes _feh, gently
        match Command::new("killall").arg("feh").output() {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        };

        if _image.code() == Some(1) {
            println!("Exiting...");
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
                eprintln!("{}", language::error(9));
                process::exit(1)
            }
        };

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == 2 {

        // _image uses swaygrab to take screenshot
        let _image = match Command::new("swaygrab").arg(temp.clone()).status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(9));
                process::exit(1)
            }
        };

        if _image.code() == Some(1) {
            println!("Exiting...");
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
                eprintln!("{}", language::error(7));
                process::exit(1)
            }
        };

        // _feh displays it
        match Command::new("feh").arg(temp.clone()).arg("-F").spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(12));
                process::exit(1)
            }
        };

        // _image lets you select
        let _image = match Command::new("gnome-screenshot")
            .args(&["-a", "-f", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(7));
                process::exit(1)
            }
        };

        // _kill closes _feh, gently
        match Command::new("killall").arg("feh").output() {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        };

        println!("gnome-screenshot doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == 1 {

        // _image uses gnome-screenshot to get current window and take screenshot
        let _image = match Command::new("gnome-screenshot")
            .args(&["-w", "-e", "shadow", "-f", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(7));
                process::exit(1)
            }
        };

        println!("gnome-screenshot doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == 2 {

        // _image uses gnome-screenshot to take screenshot
        let _image = match Command::new("gnome-screenshot")
            .args(&["-f", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(7));
                process::exit(1)
            }
        };

        println!("gnome-screenshot doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
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
                eprintln!("{}", language::error(8));
                process::exit(1)
            }
        };

        println!("spectacle doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == 1 {

        // _image uses spectacle to get current window and take screenshot
        let _image = match Command::new("spectacle")
            .args(&["-abno", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(8));
                process::exit(1)
            }
        };

        println!("spectacle doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == 2 {

        // _image uses spectacle to take screenshot
        let _image = match Command::new("spectacle")
            .args(&["-fbno", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(8));
                process::exit(1)
            }
        };

        println!("spectacle doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn maim(args: usize, temp: &str) {
    if args == 0 {

        // _before_image takes a full screenshot using maim
        match Command::new("maim")
            .args(&["--hidecursor", temp.clone()])
            .output()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(10));
                process::exit(1)
            }
        };

        // _feh displays it
        match Command::new("feh").arg(temp.clone()).arg("-F").spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(12));
                process::exit(1)
            }
        };

        // _image lets you select
        let _image = match Command::new("maim")
            .args(&["--hidecursor", "-s", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(10));
                process::exit(1)
            }
        };

        // _kill closes _feh, gently
        match Command::new("killall").arg("feh").output() {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        };

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == 1 {

        // _xdo gets the active window
        let _xdo = match Command::new("xdotool").arg("getactivewindow").output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(11));
                process::exit(1)
            }
        };

        // _image uses maim to take the window gotten from xdo
        let xdo = String::from_utf8_lossy(&_xdo.stdout);

        let _image = match Command::new("maim")
            .args(&["--hidecursor", "-i", &xdo, temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(10));
                process::exit(1)
            }
        };

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == 2 {

        // _image uses maim to take screenshot
        let _image = match Command::new("maim")
            .args(&["--hidecursor", temp.clone()])
            .status()
        {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", language::error(10));
                process::exit(1)
            }
        };

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
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
            eprintln!("{}", language::error(3));
            String::from("x11").to_lowercase()
        }
    };

    // desktop environment info gotten here
    let desktop = match env::var("DESKTOP_SESSION") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("{}", language::error(4));
            String::new()
        }
    };

    screenshot(args.clone(), temp.clone(), session.clone(), desktop.clone());
    if !tmp.exists() {
        eprintln!("{}", language::error(30));
        process::exit(1);
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
                eprintln!("{}", language::error(13));
                process::exit(1)
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

    // home gets the user's name
    let home = match env::var("HOME") {
        Ok(home) => if home.to_string().is_empty() {
            eprintln!("{}", language::error(1));
            process::exit(1)
        } else {
            home
        },
        Err(_) => {
            eprintln!("{}", language::error(1));
            process::exit(1)
        }
    };
    let mut pictures = String::from(home.clone());
    pictures.push_str("/Pictures/ShareXin");

    // _dir creates pictures dir if not already there
    let _dir = match std::fs::create_dir(pictures) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(29));
        }
    };
    let mut new_file = String::from(home);
    new_file.push_str("/Pictures/ShareXin/sharexin-");

    // time gets the time in a nice format
    let time = String::from(match time::strftime("%Y-%m-%d-%H_%M_%S", &time::now()) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(25));
            process::exit(1)
        }
    });
    new_file.push_str(&time);
    new_file.push_str(".png");

    thread::sleep(Duration::new(0, 500000000));

    // _clone copies the temp file to your home pic dir
    let _clone = match std::fs::copy(tmp.clone(), new_file) {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", language::error(30));
            return;
        }
    };
    notification::file_saved();
}
