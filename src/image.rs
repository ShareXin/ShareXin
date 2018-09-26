use error;
use notification;
use save;
use screenshot;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn screenshot_tool_selection(args: usize, temp: &str, session: String) {
    match session.as_ref() {
        "wayland" => match Command::new("gnome-screenshot").arg("--version").spawn() {
            Ok(_) => screenshot::gnome(args, temp),
            Err(_) => match Command::new("spectacle").arg("--version").spawn() {
                Ok(_) => screenshot::kde(args, temp),
                Err(_) => match Command::new("swaygrab").arg("--version").spawn() {
                    Ok(_) => screenshot::sway(args, temp),
                    Err(_) => {
                        eprintln!("{}", error::message(26));
                        notification::error(26);
                        error::fatal()
                    }
                },
            },
        },
        "x11" => match Command::new("gnome-screenshot").arg("--version").spawn() {
            Ok(_) => screenshot::gnome(args, temp),
            Err(_) => match Command::new("spectacle").arg("--version").spawn() {
                Ok(_) => screenshot::kde(args, temp),
                Err(_) => match Command::new("scrot").arg("--version").spawn() {
                    Ok(_) => screenshot::scrot(args, temp),
                    Err(_) => {
                        eprintln!("{}", error::message(26));
                        notification::error(26);
                        error::fatal()
                    }
                },
            },
        },
        _ => match Command::new("gnome-screenshot").arg("--version").spawn() {
            Ok(_) => screenshot::gnome(args, temp),
            Err(_) => match Command::new("spectacle").arg("--version").spawn() {
                Ok(_) => screenshot::kde(args, temp),
                Err(_) => match Command::new("scrot").arg("--version").spawn() {
                    Ok(_) => screenshot::scrot(args, temp),
                    Err(_) => {
                        eprintln!("{}", error::message(26));
                        notification::error(26);
                        error::fatal()
                    }
                },
            },
        },
    }
}

pub fn image(args: usize) {
    // tmp gets the temporary directory of the system
    let tmp = temp_dir(0);

    // makes a string
    let temp = tmp.to_str().unwrap();

    // Wayland/X11 session info gotten here
    let session = match env::var("XDG_SESSION_TYPE") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => String::new(),
    };

    screenshot_tool_selection(args.clone(), temp.clone(), session.clone());

    if !tmp.is_file() {
        eprintln!("{}", error::message(30));
        error::fatal();
    }

    save::save();
}

pub fn temp_dir(option: usize) -> PathBuf {
    let mut tmp = env::temp_dir();
    if option == 0 {
        tmp.push("sharexin.png");
    } else {
        tmp.push("sharexin-tmp");
        tmp.set_extension("png");
    }
    return tmp;
}
