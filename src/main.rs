extern crate gtk;
extern crate glib;
extern crate time;
extern crate libnotify;
use gtk::*;
use libnotify::Notification;
#[allow(unused_imports)]
use std::fs;
use std::env;
use std::thread;
use std::process::Command;

fn toot_img(txt: String)
{
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    println!("Text: {}", txt);
    let _mastodon = Command::new("toot")
    .args(&["post", "-m", temp.clone(), &txt])
    .output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_mastodon.stdout));
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new("Sent to Mastodon", None, temp);
    not.show().unwrap();
    libnotify::uninit();
}

fn twitter_img(txt: String)
{
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    println!("Text: {}", txt);
    if !txt.is_empty() {
        let mut _t = Command::new("t")
        .args(&["update", &txt, "-f", temp.clone()]).output().expect("Nope");
        println!("{}", String::from_utf8_lossy(&_t.stdout));
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new("Sent to Twitter", None, temp);
        not.show().unwrap();
        libnotify::uninit();
    }
    else {
        let mut _t = Command::new("t")
        .args(&["update", "-f", temp.clone()]).output().expect("Nope");
        println!("{}", String::from_utf8_lossy(&_t.stdout));
        libnotify::init("ShareXin").unwrap();
        let not = Notification::new("Sent to Twitter", None, temp);
        not.show().unwrap();
        libnotify::uninit();
    }
}

fn toot(txt: String)
{
    println!("Text: {}", txt);
    let _mastodon = Command::new("toot")
    .args(&["post", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_mastodon.stdout));
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new("Sent to Mastodon", None, None);
    not.show().unwrap();
    libnotify::uninit();
}

fn twitter(txt: String)
{
    println!("Text: {}", txt);
    let _t = Command::new("t")
    .args(&["update", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_t.stdout));
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new("Sent to Twitter", None, None);
    not.show().unwrap();
    libnotify::uninit();
}

fn image(cmd: String)
{
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    let mut file = String::from("--file=");
    file.push_str(temp);
    if cmd == "-a" { 
        let _before_image = Command::new("gnome-screenshot")
        .arg(file.clone()).output().expect("Nope");
        let _feh = Command::new("feh").arg(temp).arg("-F")
        .spawn().expect("Nope");
        let _image = Command::new("gnome-screenshot").arg(&cmd)
        .arg(file.clone()).output().expect("Nope");
        println!("{}", String::from_utf8_lossy(&_image.stdout));
        let _kill = Command::new("killall").arg("feh").output().expect("Nope");
    }
    else {
        let _image = Command::new("gnome-screenshot").arg(&cmd)
        .arg(file).output().expect("Nope");
        println!("{}", String::from_utf8_lossy(&_image.stdout));
    }
    save();
}

fn save()
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
    libnotify::init("ShareXin").unwrap();
    let not = Notification::new("File saved", None, None);
    not.show().unwrap();
    libnotify::uninit();
}

fn gui(mort: bool, morti: bool)
{
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    if mort {
        window.set_title("ShareXin - Mastodon");
    }
    else {
        window.set_title("ShareXin - Twitter");
    }
    window.set_default_size(350, 250);
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    let grid = gtk::Grid::new();
    grid.set_column_spacing(5);
    grid.set_row_spacing(5);
    let text = gtk::TextView::new();
    text.set_hexpand(true);
    text.set_vexpand(true);
    text.set_wrap_mode(gtk::WrapMode::WordChar);
    text.set_accepts_tab(false);
    grid.attach(&text, 0, 0, 3, 3);
    let cancel = Button::new_with_label("Cancel");
    cancel.set_size_request(40, 30);
    let send = Button::new();
    if mort {
        let button_txt = Label::new_with_mnemonic(Some("Toot"));
        send.add(&button_txt);
    }
    else {
        let button_txt = Label::new_with_mnemonic(Some("Tweet"));
        send.add(&button_txt);
    }
    send.set_size_request(40, 30);
    grid.attach(&cancel, 1, 4, 1, 1);
    grid.attach(&send, 2, 4, 1, 1);
    window.add(&grid);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    cancel.connect_clicked(|_| {
        gtk::main_quit();
    });

    send.connect_clicked(move |_| {
        let sent: Option<String> = TextBuffer::get_text(&TextView::get_buffer(&text).unwrap(),
        &TextBuffer::get_start_iter(&TextView::get_buffer(&text).unwrap()), 
        &TextBuffer::get_end_iter(&TextView::get_buffer(&text).unwrap()), false);
        let tweet: String = sent.unwrap();
        if mort {
            if morti {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        toot_img(tweet.clone());
                        gtk::main_quit();
                        Continue(false)
                    });
                 });
            }
            else if !tweet.is_empty() {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        toot(tweet.clone());
                        gtk::main_quit();
                        Continue(false)
                    });
                 });
            }
            else {
                libnotify::init("ShareXin").unwrap();
                let not = Notification::new("Toot empty | Not Sent", None, None);
                not.show().unwrap();
                libnotify::uninit();
            }
        }
        else {
            if morti {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        twitter_img(tweet.clone());
                        gtk::main_quit();
                        Continue(false)
                    });
                 });
            }
            else if !tweet.is_empty() {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        twitter(tweet.clone());
                        gtk::main_quit();
                        Continue(false)
                    });
                 });
            }
            else {
                libnotify::init("ShareXin").unwrap();
                let not = Notification::new("Tweet empty | Not Sent", None, None);
                not.show().unwrap();
                libnotify::uninit();
            }
        }
       window.hide();
    });

    gtk::main();
}

fn main()
{
    let version = String::from("sharexin 0.2.7");
    let help = String::from(
"ShareXin

Usage:
    sharerust [options]
    sharerust -at
    sharerust --help
    sharerust -wm
    sharerust -m

Image Options:
    -h, --help\t\tDisplay this help message
    -V, --version\tPrint version info and exit
    -a\t\t\tCapture an area (default is Fullscreen)
    -w\t\t\tCapture the current window (default is Fullscreen)
    -n\t\t\tNo Image will be taken, will tweet without an image

Social Options:
    -m\t\tUpload to Mastodon (uses \"toot\")
    -t\t\tUpload to Twitter (uses \"t\")
    -f\t\tOnly save file");
        let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_ref() {
            "-h" | "--help" | "-a" | "-w" | "-n" => println!("{}", help),
            "-V" | "--version" => println!("{}", version),
            "-am" => {
                image(String::from("-a"));
                gui(true, true);
            },
            "-at" => {
                image(String::from("-a"));
                gui(false, true);
            },
            "-wm" => {
                image(String::from("-w"));
                gui(true, true);
            },
            "-wt" => {
                image(String::from("-w"));
                gui(false, true);
            },
            "-nt" => {
                gui(false, false);
            },
            "-nm" => {
                gui(true, false);
            },
            "-m" => {
                image(String::new());
                gui(true, true);
            },
            "-t" => {
                image(String::new());
                gui(false, true);
            },
            "-af" => {
                image(String::from("-a"));
                save();
            },
            "-wf" => {
                image(String::from("-w"));
                save();
            },
            "-f" => {
                image(String::new());
                save();
            },
            _ => println!("{}", help)
        }
    }
    else {
        println!("{}", help);
    }
}
