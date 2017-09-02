use glib::{Continue, idle_add};
use std::{process, thread, time};
use sharexin;
use error;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use systray;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub fn tray() {
    loop {
        let mut app = match systray::Application::new() {
            Ok(w) => w,
            Err(_) => {
                eprintln!("{}", error::message(31));
                error::fatal();
            }
        };
        app.set_icon_from_file(&"/usr/share/icons/hicolor/48x48/apps/gedit.png".to_string())
            .ok();
        app.add_menu_item(&"Tweet".to_string(), move |_| {
            idle_add(move || {
                sharexin::tweet();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Toot".to_string(), move |_| {
            idle_add(move || {
                sharexin::toot();
                Continue(false)
            });
        }).ok();
        app.add_menu_separator().ok();
        app.add_menu_item(&"Area - Tweet".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::tweet_area();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Area - Toot".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::toot_area();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Window - Tweet".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::tweet_window();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Window - Toot".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::toot_window();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Full - Tweet".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::tweet_full();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Full - Toot".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::toot_full();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Area - Imgur".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::imgur_area();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Window - Imgur".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::imgur_window();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Full - Imgur".to_string(), move |_| {
            idle_add(move || {
                wait();
                sharexin::imgur_full();
                Continue(false)
            });
        }).ok();
        app.add_menu_item(&"Quit".to_string(), |window| {
            window.quit();
            process::exit(0);
        }).ok();
        app.wait_for_message();
    }
}

#[cfg(target_os = "macos")]
pub fn tray() {
    eprintln!("{}", error::message(31));
    error::fatal();
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn wait() {
    thread::sleep(time::Duration::new(0, 600000000));
}
