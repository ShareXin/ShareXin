use std::env;
use error;
use notification;

pub fn session() -> String {

    match env::var("XDG_SESSION_TYPE") {
        Ok(ok) => ok.to_lowercase(),
        Err(_) => {
            eprintln!("{}", error::message(3));
            String::new()
        }
    }

}

pub fn desktop() -> String {

    let desktop = 
    (match env::var("XDG_CURRENT_DESKTOP") {
        Ok(ok) => ok,
        Err(_) => String::new()
    },
    match env::var("XDG_SESSION_DESKTOP") {
        Ok(ok) => ok,
        Err(_) => String::new()
    },
    match env::var("DESKTOP_SESSION") {
        Ok(ok) => ok,
        Err(_) => String::new()
    });
    println!("{:?}", desktop);

    return String::new()

}
