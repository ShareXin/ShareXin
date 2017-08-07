// many _ are used in rust to make compiler ignore "unused_variable" tips

// args may be "-s" for selection screenshots
//     "-i" for window screenshots, or empty for fullscreenshots

#[cfg(target_os = "macos")]
use screenshot::get_screenshot;

use std::time::Duration;
use std::process::*;
use std::*;
use std;
use notification;
use time;

pub fn open(file: String) {

    // tmp gets temporary dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");

    // copy copies file to temp

    thread::sleep(Duration::new(1, 0));
    let _copy = match std::fs::copy(file, tmp.clone()) {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to save file. {:?}", e),
    };

    notification::file_saved();
}

#[cfg(target_os = "macos")]
fn screenshot(args: String, temp: &str) {

    // tmp gets temporary dir
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
#[cfg(target_family = "unix")]
fn screenshot(args: String, temp: &str) {
    // x11/wayland session info gotten here
    let mut _session = String::new();
    _session = match env::var("XDG_SESSION_TYPE") {
        Ok(ok) => ok,
        Err(e) => panic!("XDG not found. {:?}", e),
    };
    let session = &_session.to_lowercase();

    // desktop environment info gotten here
    let mut _desktop = String::new();
    _desktop = match env::var("DESKTOP_SESSION") {
        Ok(ok) => ok,
        Err(e) => panic!("XDG not found. {:?}", e),
    };
    let desktop = &_desktop.to_lowercase();

    match session.as_ref() {
        "wayland" => match desktop.as_ref() {
            "gnome" => gnome(args, temp),
            "plasma" => kde(args, temp),
            "sway" => sway(args, temp),
            _ => panic!(
                "Only Gnome/Plasma/Sway desktops supported for Wayland.
                Unable to figure out desktop."
            ),
        },
        "x11" => match desktop.as_ref() {
            "gnome" => gnome(args, temp),
            "plasma" => kde(args, temp),
            _ => maim(args, temp),
        },
        _ => panic!("Unable to figure out session type. Check XDG variable."),
    }
}

#[cfg(not(target_os = "macos"))]
#[cfg(target_family = "unix")]
fn sway(args: String, temp: &str) {
    if args == "-s" {

        // _before_image takes a full screenshot using swaygrab
        let _before_image = Command::new("swaygrab")
            .arg(temp.clone())
            .output()
            .expect("Swaygrab not found");

        // _feh displays it and sleeps the thread to wait for _image
        println!("Feh may not display properly due to tiling and Wayland.");

        let _feh = Command::new("feh")
            .arg(temp.clone())
            .arg("-F")
            .spawn()
            .expect("Feh not found");

        thread::sleep(Duration::new(0, 500000000));

        // _image lets use _slop to select
        let _slop = Command::new("slop").output().expect("slop not found");
        let slop = String::from_utf8_lossy(&_slop.stdout);
        let _image = Command::new("convert")
            .args(&[temp.clone(), "-crop", &slop, temp.clone()])
            .status()
            .expect("ImageMagick not found");

        // _kill closes _feh, gently
        let _kill = Command::new("killall")
            .arg("feh")
            .output()
            .expect("killall not found");

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == "-i" {

        // _image uses swaygrab to get "focused" window and take screenshot
        let _image = Command::new("swaygrab")
            .arg("-f")
            .arg(temp.clone())
            .status()
            .expect("swaygrab not found");

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else {

        // _image uses swaygrab to take screenshot
        let _image = Command::new("swaygrab")
            .arg(temp.clone())
            .status()
            .expect("swaygrab not found");

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    }
}

#[cfg(not(target_os = "macos"))]
#[cfg(target_family = "unix")]
fn gnome(args: String, temp: &str) {
    if args == "-s" {

        // _before_image takes a full screenshot using gnome-screenshot
        let _before_image = Command::new("gnome-screenshot")
            .arg("-f")
            .arg(temp.clone())
            .output()
            .expect("gnome-screenshot not found");

        // _feh displays it and sleeps the thread to wait for _image
        let _feh = Command::new("feh")
            .arg(temp.clone())
            .arg("-F")
            .spawn()
            .expect("Feh not found");

        thread::sleep(Duration::new(0, 500000000));

        // _image lets you select
        let _image = Command::new("gnome-screenshot")
            .arg("-a")
            .args(&["-f", temp.clone()])
            .status()
            .expect("gnome-screenshot not found");

        // _kill closes _feh, gently
        let _kill = Command::new("killall")
            .arg("feh")
            .output()
            .expect("killall not found");

        println!("gnome-screenshot doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == "-i" {

        // _image uses gnome-screenshot to get current window and take screenshot
        let _image = Command::new("gnome-screenshot")
            .arg("-w")
            .args(&["-f", temp.clone()])
            .status()
            .expect("gnome-screenshot not found");

        println!("gnome-screenshot doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else {

        // _image uses gnome-screenshot to take screenshot
        let _image = Command::new("gnome-screenshot")
            .arg("-f")
            .arg(temp.clone())
            .status()
            .expect("gnome-screenshot not found");

        println!("gnome-screenshot doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    }
}

#[cfg(not(target_os = "macos"))]
#[cfg(target_family = "unix")]
fn kde(args: String, temp: &str) {
    if args == "-s" {

        // _image pauses screen and lets you select
        let _image = Command::new("spectacle")
            .arg("-rbno")
            .arg(temp.clone())
            .status()
            .expect("spectacle not found");

        println!("spectacle doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == "-i" {

        // _image uses spectacle to get current window and take screenshot
        let _image = Command::new("spectacle")
            .arg("-abno")
            .arg(temp.clone())
            .status()
            .expect("spectacle not found");

        println!("spectacle doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else {

        // _image uses spectacle to take screenshot
        let _image = Command::new("spectacle")
            .arg("-fbno")
            .arg(temp.clone())
            .status()
            .expect("spectacle not found");

        println!("spectacle doesnt give exit codes but maybe one day");
        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    }
}

#[cfg(not(target_os = "macos"))]
#[cfg(target_family = "unix")]
fn maim(args: String, temp: &str) {
    if args == "-s" {

        // _before_image takes a full screenshot using maim
        let _before_image = Command::new("maim")
            .arg("-u")
            .arg(temp.clone())
            .output()
            .expect("maim not found");

        // _feh displays it and sleeps the thread to wait for _image
        let _feh = Command::new("feh")
            .arg(temp.clone())
            .arg("-F")
            .spawn()
            .expect("Feh not found");

        thread::sleep(Duration::new(0, 500000000));

        // _image lets you select
        let _image = Command::new("maim")
            .arg("-su")
            .arg(temp.clone())
            .status()
            .expect("maim not found");

        // _kill closes _feh, gently
        let _kill = Command::new("killall")
            .arg("feh")
            .output()
            .expect("killall not found");

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else if args == "-i" {

        // _xdo gets the active window
        let _xdo = Command::new("xdotool")
            .arg("getactivewindow")
            .output()
            .expect("xdotool not found");

        // _image uses maim to take the window gotten from xdo
        let xdo = String::from_utf8_lossy(&_xdo.stdout);

        let _image = Command::new("maim")
            .arg("-ui")
            .args(&[&xdo, temp.clone()])
            .status()
            .expect("maim not found");

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    } else {

        // _image uses maim to take screenshot
        let _image = Command::new("maim")
            .arg("-u")
            .arg(temp.clone())
            .status()
            .expect("maim not found");

        if _image.code() == Some(1) {
            println!("Exiting...");
            process::exit(1);
        }
    }
}

pub fn image(args: String) {
    //  tmp gets the temporary directory of the system
    let mut tmp = env::temp_dir();

    // adds filename
    tmp.push("sharexin.png");

    // makes a string
    let temp = tmp.to_str().unwrap();

    screenshot(args.clone(), temp.clone());
    if !tmp.exists() {
        panic!("File not saved");
    }

    if args == "-i" {
        //  adds a shadow
        let _ = Command::new("convert")
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
            .spawn()
            .expect("ImageMagick not found");
    }

    save();
}

pub fn save() {
    // tmp gets temporary dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");

    // home gets the user's name
    let home = match env::var("HOME") {
        Ok(home) => home,
        Err(e) => panic!("Could not read $HOME. {:?}", e),
    };
    let mut pictures = String::from(home.clone());
    pictures.push_str("/Pictures/ShareXin");

    // _dir creates pictures dir if not already there
    let _dir = match std::fs::create_dir(pictures) {
        Ok(ok) => ok,
        Err(e) => print!("Unable to create folder. {:?}", e),
    };
    let mut new_file = String::from(home);
    new_file.push_str("/Pictures/ShareXin/sharexin-");

    // time gets the time in a nice format
    let time = String::from(match time::strftime("%Y-%m-%d-%H_%M_%S", &time::now()) {
        Ok(ok) => ok,
        Err(e) => panic!("Couldn't get time. {:?}", e),
    });
    new_file.push_str(&time);
    new_file.push_str(".png");

    // _clone copies the temp file to your home pic dir
    thread::sleep(Duration::new(1, 0));
    let _clone = match std::fs::copy(tmp.clone(), new_file) {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to save file. {:?}", e),
    };
    notification::file_saved();
}
