extern crate libnotify;
extern crate pipers;
extern crate time;
use pipers::Pipe;
use libnotify::Notification;
use std::env;
use std;
use std::process::*;

pub fn image(cmd: String)
{
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    if cmd == "-s" { 
        let _before_image = Command::new("maim")
        .arg(temp.clone()).output().expect("Nope");
        println!("[Before Image] {}", String::from_utf8_lossy(&_before_image.stdout));
        let _feh = Command::new("feh").arg(temp.clone()).arg("-F")
        .spawn().expect("Nope");
        let _sleep = Command::new("sleep").arg("1").output().expect("Nope");
        println!("[Sleep] {}", String::from_utf8_lossy(&_sleep.stdout));
        let _image = Pipe::new("maim -s")
        .then("convert - ( +clone -background black -shadow 80x3+5+5 ) +swap -background none -layers merge +repage /tmp/sharexin.png")
        .finally()
        .expect("Nope")
        .wait_with_output()
        .expect("NopeNope");
        let _kill = Command::new("killall").arg("feh").output().expect("Nope");
        println!("[Feh Kill] {}", String::from_utf8_lossy(&_kill.stdout));
    }
    else if cmd == "-i" {
        let _image = Command::new("maim")
        .args(&[&cmd, "$(xdotool getactivewindow)", temp])
        .output().expect("Nope");
        println!("[Window Image] {}", String::from_utf8_lossy(&_image.stdout));
    }
    else {
        let _image = Command::new("maim")
        .arg(temp).output().expect("Nope");
        println!("[Full Image] {}", String::from_utf8_lossy(&_image.stdout));
    }
    save();
}

pub fn save()
{
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let username = env::var("USER").unwrap();
    let mut pictures = String::from("/home/");
    pictures.push_str(&username);
    pictures.push_str("/Pictures/ShareXin");
    #[allow(unused_must_use)]
    let _ = std::fs::create_dir(pictures);
    println!("[Creating Folder]");
    let mut new_file = String::from("/home/");
    new_file.push_str(&username);
    new_file.push_str("/Pictures/ShareXin/sharexin-");
    let time = String::from(time::strftime("%Y-%m-%d-%T", &time::now()).unwrap());
    new_file.push_str(&time);
    new_file.push_str(".png");
    #[allow(unused_must_use)]
    let _ = std::fs::copy(tmp.clone(), new_file);
    println!("[Saving image]");
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new("File saved", None, None);
    not.show().unwrap();
    libnotify::uninit();
    println!("[Notification]");
}
