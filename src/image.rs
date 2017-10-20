use std::time::Duration;
use std::process::Command;
use std::path::PathBuf;
use std::{env, thread};
use notification;
use error;
use save;
use desktop;
use screenshot;

fn screenshot(args: usize, temp: &str, session: String, desktop: String) {

    match session.as_ref() {
        "wayland" => {
            match desktop.as_ref() {
                "gnome" => screenshot::gnome(args, temp),
                "cinnamon" => screenshot::gnome(args, temp),
                "budgie-desktop" => screenshot::gnome(args, temp),
                "budgie:gnome" => screenshot::gnome(args, temp),
                "ubuntu" => screenshot::gnome(args, temp),
                "ubuntu:gnome" => screenshot::gnome(args, temp),
                "plasma" => screenshot::kde(args, temp),
                "kde" => screenshot::kde(args, temp),
                "sway" => screenshot::sway(args, temp),
                _ => {
                    eprintln!("{}", error::message(26));
                    notification::error(26);
                    error::fatal()
                }
            }
        }
        "x11" => {
            match desktop.as_ref() {
                "gnome" => screenshot::gnome(args, temp),
                "cinnamon" => screenshot::gnome(args, temp),
                "x-cinnamon" => screenshot::gnome(args, temp),
                "ubuntu" => screenshot::gnome(args, temp),
                "unity:unity7" => screenshot::gnome(args, temp),
                "budgie-desktop" => screenshot::gnome(args, temp),
                "budgie:gnome" => screenshot::gnome(args, temp),
                "plasma" => screenshot::kde(args, temp),
                "kde" => screenshot::kde(args, temp),
                _ => screenshot::scrot(args, temp),
            }
        }
        _ => {
            match desktop.as_ref() {
                "gnome" => screenshot::gnome(args, temp),
                "cinnamon" => screenshot::gnome(args, temp),
                "x-cinnamon" => screenshot::gnome(args, temp),
                "ubuntu" => screenshot::gnome(args, temp),
                "unity:unity7" => screenshot::gnome(args, temp),
                "budgie-desktop" => screenshot::gnome(args, temp),
                "budgie:gnome" => screenshot::gnome(args, temp),
                "plasma" => screenshot::kde(args, temp),
                "kde" => screenshot::kde(args, temp),
                "macos" => screenshot::mac(args, temp),
                _ => screenshot::scrot(args, temp),
            }
        }
    }
}

pub fn image(args: usize) {

    // tmp gets the temporary directory of the system
    let tmp = temp_dir(0);

    // makes a string
    let temp = tmp.to_str().unwrap();

    // x11/wayland session info gotten here
    let session = desktop::session();

    // desktop environment info gotten here
    let desktop = desktop::desktop();

    screenshot(args.clone(), temp.clone(), session.clone(), desktop.clone());

    if !tmp.is_file() {
        eprintln!("{}", error::message(30));
        error::fatal();
    }

    if args == 1 && desktop != "gnome" && !cfg!(target_os = "macos") {
        //  adds a shadow
        match Command::new("convert")
            .arg(temp.clone())
            .args(
                &[
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
                ],
            )
            .arg(temp.clone())
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(13));
                notification::error(13);
                error::fatal()
            }
        };
        thread::sleep(Duration::new(0, 500000000));
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
