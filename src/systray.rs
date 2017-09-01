use systray;

fn systray() {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create systray!")
    }
    app.set_icon_from_file(&"/usr/share/gxkb/flags/ua.png".to_string()).ok();
    app.add_menu_item(&"Window".to_string(), |_| {
        // Nothing
    }).ok();
    app.add_menu_item(&"Tweet".to_string(), |window| {
        window.add_menu_item(&"Interior item".to_string(), |_| {
            // Tweet window screenshot
        }).ok();
        window.add_menu_separator().ok();
    }).ok();
    app.add_menu_separator().ok();
    app.add_menu_item(&"Quit".to_string(), |window| {
        window.quit();
    }).ok();
    app.wait_for_message();
}
