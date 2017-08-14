use std::env;
use std::process::Command;
use notification;
use Destination;
use error;

pub fn image(txt: String) {
    let mastodon = Destination::new(0);

    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();

    let _toot = match Command::new("toot")
        .args(&["post", "-m", temp.clone(), &txt])
        .status()
    {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            notification::not_sent(mastodon);
            error::fatal()
        }
    };
    if _toot.code() == Some(2) {
        eprintln!("{}", error::message(21));
        notification::not_sent(mastodon);
        error::fatal();
    }
    notification::image_sent(mastodon, &txt, temp);
}

pub fn toot(txt: String) {
    let mastodon = Destination::new(0);

    let _toot = match Command::new("toot").args(&["post", &txt]).status() {
        Ok(ok) => ok,
        Err(_) => {
            eprintln!("{}", error::message(6));
            notification::not_sent(mastodon);
            error::fatal()
        }
    };
    if _toot.code() == Some(2) {
        eprintln!("{}", error::message(21));
        notification::not_sent(mastodon);
        error::fatal();
    }
    notification::message_sent(mastodon, &txt);
}
