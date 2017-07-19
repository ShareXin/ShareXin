extern crate gtk;
extern crate glib;
extern crate notify_rust;
extern crate time;
extern crate users;
use users::{get_user_by_uid, get_current_uid};
use gtk::*;
#[warn(unused_imports)]
use std::fs;
use std::env;
use std::thread;
use std::process::Command;
use notify_rust::Notification;

fn toot(txt: String)
{
    println!("Text: {}", txt);
    let mastodon = Command::new("toot")
    .arg("post").arg("-m").arg("/tmp/sharexin_img.png").arg(&txt)
    .output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&mastodon.stdout));
    Notification::new().summary("Sent to Mastodon")
        .body(&txt).icon("file:///tmp/sharexin_img.png").show().unwrap();
}

fn twitter(txt: String)
{
    println!("Text: {}", txt);
    let t = Command::new("t")
    .arg("update").arg("-f").arg("/tmp/sharexin_img.png").arg(&txt)
    .output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&t.stdout));
    Notification::new().summary("Sent to Twitter")
        .body(&txt).icon("file:///tmp/sharexin_img.png").show().unwrap();
}

fn image(cmd: String)
{
    let image = Command::new("gnome-screenshot").arg(&cmd)
        .arg("--file=/tmp/sharexin_img.png").output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&image.stdout));
}

fn save()
{
    #[warn(unused_must_use)]
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = String::from(user.name());
    let mut pictures = String::from("/home/");
    pictures.push_str(&username);
    pictures.push_str("/Pictures/ShareXin");
    std::fs::create_dir(pictures);
    let mut new_file = String::from("/home/jorge/Pictures/ShareXin/sharexin-");
    let time = String::from(time::strftime("%Y-%m-%d-%T", &time::now()).unwrap());
    new_file.push_str(&time);
    new_file.push_str(".png");
    std::fs::copy("/tmp/sharexin_img.png", new_file);
}

fn gui(mort: bool)
{
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("ShareXin");
    window.set_default_size(350, 250);
    window.set_border_width(10);
    let grid = gtk::Grid::new();
    grid.set_column_spacing(5);
    grid.set_row_spacing(5);
    let text = gtk::TextView::new();
    text.set_hexpand(true);
    text.set_vexpand(true);
    grid.attach(&text, 0, 0, 3, 3);
    let cancel = Button::new_with_label("Cancel");
    cancel.set_size_request(40, 30);
    let send = Button::new_with_label("Send");
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
        Notification::new().summary("File saved")
            .icon("file:///tmp/sharexin_img.png").show().unwrap();
        gtk::main_quit();
    });

    send.connect_clicked(move |_| {
        let sent: Option<String> = TextBuffer::get_text(&TextView::get_buffer(&text).unwrap(),
        &TextBuffer::get_start_iter(&TextView::get_buffer(&text).unwrap()), 
        &TextBuffer::get_end_iter(&TextView::get_buffer(&text).unwrap()), false);
        let tweet: String = sent.unwrap();
        if mort {
        thread::spawn(move || {
            glib::idle_add(move || {
                toot(tweet.clone());
                gtk::main_quit();
                Continue(false)
            });
        });}
        else {
        thread::spawn(move || {
            glib::idle_add(move || {
                twitter(tweet.clone());
                gtk::main_quit();
                Continue(false)
            });
        });}
       window.hide();
    });

    gtk::main();
}

fn main()
{
    let help = "
Usage: sharerust [OPTION (Image,Social)]
Examples: 
sharerust -at
sharerust --help
sharerust -wm

Image Options:
\t-h --help\tDisplay this message
\t-a\t\tCapture an area (default is Fullscreen)
\t-w\t\tCapture the current window (default is Fullscreen)

Social Options:
\t-m\t\tUpload to Mastodon (uses \"toot\")
\t-t\t\tUpload to Twitter (uses \"t\")";
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            println!("{}", help);
        }
        else if args[1] == "-a" || args[1] == "--area" {
            println!("{}", help);
        }
        else if args[1] == "-am" {
            image(String::from("-a"));
            gui(true);
        }
        else if args[1] == "-at" {
            image(String::from("-a"));
            gui(false);
        }
        else if args[1] == "-wm" {
            image(String::from("-w"));
            gui(true);
        }
        else if args[1] == "-wt" {
            image(String::from("-w"));
            gui(false);
        }
        else if args[1] == "-m" {
            image(String::from(""));
            gui(true);
        }
        else if args[1] == "-t" {
            image(String::from(""));
            gui(false);
        }
        else if args[1] == "-af" {
            image(String::from("-a"));
            save();
        }
        else if args[1] == "-wf" {
            image(String::from("-w"));
            save();
        }
        else if args[1] == "-f" {
            image(String::from(""));
            save();
        }
        else {
            println!("Unknown option. Use the --help flag for Help.");
        }
    }
    if args.len() == 1 {
        println!("No option specified. Use the --help flag for Help.");
    }
}