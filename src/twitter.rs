use std::*;
use std::process::*;
use notification;
use Destination;
use error;

pub fn image(txt: String) {
    let twitter = Destination::new(1);

    // gets file to send from temp dir
    let mut tmp = env::temp_dir();
    tmp.push("sharexin.png");
    let temp = tmp.to_str().unwrap().clone();

    let _t = match Command::new("t")
        .args(&["update", &txt, "-f", temp.clone()])
        .status()
    {
        Ok(ok) => ok,
        Err(_) => {
            println!("{}", error::message(4));
            notification::not_sent(twitter);
            process::exit(1)
        }
    };
    Command::new("killall").arg("vim");
    if _t.code() == Some(1) {
        println!("{}", error::message(27));
        process::exit(1);
    }
    notification::image_sent(twitter, &txt, temp);
}

pub fn tweet(txt: String) {
    let twitter = Destination::new(1);

    let _t = match Command::new("t").args(&["update", &txt]).status() {
        Ok(ok) => ok,
        Err(_) => {
            println!("{}", error::message(4));
            notification::not_sent(twitter);
            process::exit(1)
        }
    };
    Command::new("killall").arg("vim");
    if _t.code() == Some(1) {
        println!("{}", error::message(27));
        process::exit(1);
    }
    notification::message_sent(twitter, &txt);
}
