use std::env;
use error;
use notification;

pub fn session() -> String {

    match env::var("XDG_SESSION_TYPE") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("{}", error::message(3));
            notification::error(3);
            String::new()
        }
    }

}

pub fn desktop() -> String {

    let desktop = match env::var("DESKTOP_SESSION") {
        Ok(ok) => {
            println!("{}", ok);
            if ok.contains("/") {
                match env::var("XDG_SESSION_DESKTOP") {
                    Ok(o) => {
                        println!("{}", o);
                        o
                    }
                    Err(_) => {
                        match env::var("XDG_CURRENT_DESKTOP") {
                            Ok(k) => {
                                println!("{}", k);
                                k
                            }
                            Err(_) => {
                                eprintln!("{}", error::message(4));
                                notification::error(4);
                                String::new()
                            }
                        }
                    }
                }
            } else {
                ok
            }
        }
        Err(_) => {
            if !cfg!(target_os = "macos") {
                match env::var("XDG_SESSION_DESKTOP") {
                    Ok(ok) => {
                        println!("{}", ok);
                        ok
                    }
                    Err(_) => {
                        match env::var("XDG_CURRENT_DESKTOP") {
                            Ok(o) => {
                                println!("{}", o);
                                o
                            }
                            Err(_) => {
                                eprintln!("{}", error::message(4));
                                notification::error(4);
                                String::new()
                            }
                        }
                    }
                }
            } else if cfg!(target_os = "macos") {
                String::from("macos")
            } else {
                notification::error(4);
                String::new()
            }
        }
    };
    println!("{}", desktop);
    return desktop.to_lowercase();

}
