extern crate time;
use std::time::Duration;
use std::process::*;
use std::*;
use std;
use notification;

pub fn open(file: String)
{
    //tmp gets temporary dir

    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");

    //_ copies file to temp

    let _sleep = Duration::new(1,0);
    thread::sleep(_sleep);
    let _ = match std::fs::copy(file, tmp.clone()) {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to save file. {:?}", e),
    };
}

pub fn image(cmd: String)
{
    //tmp gets the temporary directory of the system

    let mut tmp = env::temp_dir();
    //adds filename

    tmp.push("sharexin.png");
    //makes a string

    let temp = tmp.to_str().unwrap().clone();
    if cmd == "-s" {

        //_before_image takes a full screenshot using maim

        let _before_image = Command::new("maim")
        .arg(temp.clone()).output().expect("Nope");

        //_feh displays it and _sleeps waits for _image

        let _feh = Command::new("feh").arg(temp.clone()).arg("-F")
        .spawn().expect("Nope");
        let _sleep = Command::new("sleep").arg("0.5").output().expect("Nope");

        //_image lets to select

        let _image = Command::new("maim").arg("-s").arg(temp.clone())
        .output().expect("Nope");

        //_convert_area adds a shadow

        let _convert_area = Command::new("convert").arg(temp.clone())
        .args(&["(", "+clone", "-background", "black", "-shadow", "80x3+5+5"])
        .args(&[")", "+swap", "-background", "none", "-layers", "merge", "+repage"])
        .arg(temp.clone()).spawn().expect("Nope");

        //double shadow cause mac dev amiright

        let _ = Command::new("convert").arg(temp.clone())
        .args(&["(", "+clone", "-background", "black", "-shadow", "80x3+5+5"])
        .args(&[")", "+swap", "-background", "none", "-layers", "merge", "+repage"])
        .arg(temp.clone()).spawn().expect("Nope");

        //_kill closes _feh, gently

        let _kill = Command::new("killall").arg("feh").output().expect("Nope");
    }
    else if cmd == "-i" {

        //_xdo gets the active window

        let _xdo = Command::new("xdotool").arg("getactivewindow")
        .output().expect("Nope");

        //_image uses maim to take the window gotten from xdo

        let xdo = String::from_utf8_lossy(&_xdo.stdout);
        let _image = Command::new("maim").arg("-i")
        .args(&[&xdo, temp.clone()]).output().expect("Nope");
        
        //_convert_window adds shadow

        let _convert_window = Command::new("convert").arg(temp.clone())
        .args(&["(", "+clone", "-background", "black", "-shadow", "80x3+5+5"])
        .args(&[")", "+swap", "-background", "none", "-layers", "merge", "+repage"])
        .arg(temp.clone()).spawn().expect("Nope");

        //double shadow cause mac dev amiright
        let _ = Command::new("convert").arg(temp.clone())
        .args(&["(", "+clone", "-background", "black", "-shadow", "80x3+5+5"])
        .args(&[")", "+swap", "-background", "none", "-layers", "merge", "+repage"])
        .arg(temp.clone()).spawn().expect("Nope");
    }
    else {

        //_image uses maim to take screenshot

        let _image = Command::new("maim")
        .arg(temp.clone()).output().expect("Nope");
    }
    save();
}

pub fn save()
{
    //tmp gets temporary dir

    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    //home gets the user's name
    
    let home = match env::var("HOME") {
        Ok(home) => home,
        Err(e) => panic!("Could not read $HOME. {:?}", e),
    };
    let mut pictures = String::from(home.clone());
    pictures.push_str("/Pictures/ShareXin");
    //_ creates pictures dir if not already there

    let _ = match std::fs::create_dir(pictures) {
        Ok(ok) => ok,
        Err(e) => print!("Unable to create folder. {:?}", e),
    };
    let mut new_file = String::from(home);
    new_file.push_str("/Pictures/ShareXin/sharexin-");
    //time gets the time in a nice format

    let time = String::from(
    match time::strftime("%Y-%m-%d-%T", &time::now()) {
        Ok(ok) => ok,
        Err(e) => panic!("Couldn't get time. {:?}", e),
    });
    new_file.push_str(&time);
    new_file.push_str(".png");
    //_ copies the temp file to your home pic dir

    let _sleep = Duration::new(1,0);
    thread::sleep(_sleep);
    let _ = match std::fs::copy(tmp.clone(), new_file) {
        Ok(ok) => ok,
        Err(e) => panic!("Unable to save file. {:?}", e),
    };
    notification::file_saved();
}
