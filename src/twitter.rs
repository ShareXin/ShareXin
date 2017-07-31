use std::env;
use std::process::*;
use notification;
use Destination;
use std::io::*;
use std::fs::File;
use gtk::*;
use gtk;
use std::*;
use open;

pub fn image(txt: String)
{
    let twitter = Destination::new(1);
    //gets file to send from temp dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();
    let text: &str = &txt.clone()[..];
    println!("[Tweet]: {}", txt);
    //t has troubles with empty args, so if txt is empty, it wont be send
    if !txt.is_empty() {
        let _ = Command::new("t")
        .args(&["update", &txt, "-f", temp.clone()]).spawn().expect("Nope");
        notification::image_sent(twitter, text, temp);
    }
    else {
        let _ = Command::new("t")
        .args(&["update", "-f", temp.clone()]).spawn().expect("Nope");
        notification::image_sent(twitter, text, temp);
    }
}

pub fn tweet(txt: String)
{
    let twitter = Destination::new(1);
    let text: &str = &txt.clone()[..];
    println!("[Tweet]: {}", txt);
    let _t = Command::new("t")
    .args(&["update", &txt]).output().expect("Nope");
    println!("{}", String::from_utf8_lossy(&_t.stdout));
    notification::message_sent(twitter, text);
}




fn gui()
{
    //if gtk dont init, ends program

    match gtk::init() {
        Ok(ok) => ok,
        Err(e) => panic!("GTK could not initialize. {:?}", e),
    };

    //all theses get objects from a glade builder

    let builder = gtk::Builder::new_from_string(include_str!("auth.glade"));
    let window: gtk::Window = builder.get_object("window").unwrap();
    #[allow(unused_variables)]
    let entry: gtk::Entry = builder.get_object("entry").unwrap();
    let ok: gtk::Button = builder.get_object("ok").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    ok.connect_clicked(move |_| {
        gtk::main_quit();
    });

    window.show_all();
    gtk::main();
}