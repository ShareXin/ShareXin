extern crate time;
use pipers::Pipe;
use std::env;
use std;
use std::process::*;
use send;

pub fn image(cmd: String)
{
    let mut convert_area: String =
    "convert - ( +clone -background black -shadow 80x3+5+5 ) +swap -background none -layers merge +repage ".to_owned();
    let mut convert_window: String =
    "convert ".to_owned();
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    convert_area.push_str(temp.clone());
    convert_window.push_str(temp.clone());
    convert_window.push_str(" ");
    convert_window.push_str("( +clone -background black -shadow 80x3+5+5 ) +swap -background none -layers merge +repage ");
    convert_window.push_str(temp.clone());
    if cmd == "-s" {
        let _before_image = Command::new("maim")
        .arg(temp.clone()).output().expect("Nope");
        println!("{}", String::from_utf8_lossy(&_before_image.stdout));
        let _feh = Command::new("feh").arg(temp.clone()).arg("-F")
        .spawn().expect("Nope");
        let _sleep = Command::new("sleep").arg("0.5").output().expect("Nope");
        println!("{}", String::from_utf8_lossy(&_sleep.stdout));
        let _image = Pipe::new("maim -s")
        .then(&convert_area)
        .finally()
        .expect("Nope")
        .wait_with_output()
        .expect("NopeNope");
        let _kill = Command::new("killall").arg("feh").output().expect("Nope");
        println!("{}", String::from_utf8_lossy(&_kill.stdout));
    }
    else if cmd == "-i" {
        let _xdo = Command::new("xdotool").arg("getactivewindow")
        .output().expect("Nope");
        let mut _before_command = String::from("maim -i ");
        _before_command.push_str(&String::from_utf8_lossy(&_xdo.stdout));
        _before_command.push_str(" ");
        _before_command.push_str(&temp.clone());
        let _before_image = Pipe::new(&_before_command)
        .finally().expect("Nope").wait_with_output().expect("Nope");
        let _image = Pipe::new(&convert_window)
        .finally().expect("Nope").wait_with_output().expect("NopeNope");
    }
    else {
        let _image = Command::new("maim")
        .arg(temp).output().expect("Nope");
        println!("{}", String::from_utf8_lossy(&_image.stdout));
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
    let mut new_file = String::from("/home/");
    new_file.push_str(&username);
    new_file.push_str("/Pictures/ShareXin/sharexin-");
    let time = String::from(time::strftime("%Y-%m-%d-%T", &time::now()).unwrap());
    new_file.push_str(&time);
    new_file.push_str(".png");
    #[allow(unused_must_use)]
    let _ = std::fs::copy(tmp.clone(), new_file);
    send::notification_2();
}
