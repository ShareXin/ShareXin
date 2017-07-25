extern crate gtk;
extern crate glib;
extern crate time;
extern crate libnotify;
extern crate pipers;
use libnotify::Notification;
use gtk::*;
use std::*;
mod send;
mod file;
mod help;

static VERSION: &'static str = "sharexin 0.3.1";

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
                        send::toot_img(tweet.clone());
                        gtk::main_quit();
                        Continue(false)
                    });
                 });
            }
            else if !tweet.is_empty() {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        send::toot(tweet.clone());
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
                gtk::main_quit();
            }
        }
        else {
            if morti {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        send::twitter_img(tweet.clone());
                        gtk::main_quit();
                        Continue(false)
                    });
                 });
            }
            else if !tweet.is_empty() {
                thread::spawn(move || {
                    glib::idle_add(move || {
                        send::twitter(tweet.clone());
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
                gtk::main_quit();
            }
        }
       window.hide();
    });

    gtk::main();
}

fn main()
{
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_ref() {
            "-h" | "--help" | "-a" | "-w" | "-n" => help::help(),
            "-V" | "--version" => println!("{}", VERSION),
            "-am" => {
                file::image(String::from("-s"));
                gui(true, true);
            },
            "-at" => {
                file::image(String::from("-s"));
                gui(false, true);
            },
            "-wm" => {
                file::image(String::from("-i"));
                gui(true, true);
            },
            "-wt" => {
                file::image(String::from("-i"));
                gui(false, true);
            },
            "-nt" => {
                gui(false, false);
            },
            "-nm" => {
                gui(true, false);
            },
            "-m" => {
                file::image(String::new());
                gui(true, true);
            },
            "-t" => {
                file::image(String::new());
                gui(false, true);
            },
            "-af" => {
                file::image(String::from("-s"));
            },
            "-wf" => {
                file::image(String::from("-i"));
            },
            "-f" => {
                file::image(String::new());
            },
            _ => help::help()
        }
    }
    else {
        help::help();
    }
}
